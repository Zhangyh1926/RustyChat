// src/friend_list.rs
use futures_util::{stream::SplitSink, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::types::{FriendCenterItem, Response};

pub async fn handle_friend_center_request(
    user_id: i32,
    access_token_for_check: &str,
    write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    pool: &deadpool_postgres::Pool,
    pool_redis: &deadpool_redis::Pool,
) {
    if !crate::check_token::check_token(user_id, access_token_for_check, &pool_redis, write).await {
        return;
    }
    match get_friend_center(user_id, &pool).await {
        Ok(list) => {
            let success_response = Response::FriendCenterResponse {
                status: "success".to_string(),
                message: "get friend center successful".to_string(),
                friends: Some(list),
            };
            let response_text = serde_json::to_string(&success_response).unwrap();
            write
                .send(Message::Text(response_text))
                .await
                .unwrap();
        }
        Err(err_msg) => {
            let fail_response = Response::FriendCenterResponse {
                status: "error".to_string(),
                message: err_msg,
                friends: None,
            };
            let response_text = serde_json::to_string(&fail_response).unwrap();
            write
                .send(Message::Text(response_text))
                .await
                .unwrap();
        }
    }
}

async fn get_friend_center(user_id: i32, pool: &deadpool_postgres::Pool) -> Result<Vec<FriendCenterItem>, String> {
    let client = pool
        .get()
        .await
        .map_err(|_| "Failed to connect to database".to_string())?;
    let stmt = client
        .prepare(
            "SELECT 
                u.id,
                u.username,
                u.avatar_url,
                u.signature,
                f.status
            FROM 
                users u
            JOIN 
                friends f ON (u.id = f.friend_id AND f.user_id = $1)
                    "
        )
        .await
        .map_err(|_| "Database error".to_string())?;

    let rows = client
        .query(&stmt, &[&user_id])
        .await
        .map_err(|_| "Query failed".to_string())?;

    let friend_center_items: Vec<FriendCenterItem> = rows
        .iter()
        .map(|row: &tokio_postgres::Row| -> FriendCenterItem {
            let friend_id: i32 = row.get(0);
            let friend_name: String = row.get(1);
            let avatar_url: Option<String> = row.get(2);
            let signature: Option<String> = row.get(3);
            let status: String = row.get(4);
            FriendCenterItem {
                friend_id,
                friend_name,
                avatar_url,
                signature,
                status,
            }
        })
        .collect();
    Ok(friend_center_items)
}
