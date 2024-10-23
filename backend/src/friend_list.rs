// src/friend_list.rs
use futures_util::{stream::SplitSink, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
use std::time::SystemTime;
use crate::check_token::check_token;
use crate::types::{FriendListItem, Response};

pub async fn handle_friendlist_request(
    user_id: i32,
    access_token_for_check: &str,
    write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    pool_pg: &deadpool_postgres::Pool,
    pool_redis: &deadpool_redis::Pool,
) {
    if !check_token("FriendListResponse", user_id, access_token_for_check, &pool_redis, write).await {
        return;
    }
    match get_friend_list(user_id, &pool_pg).await {
        Ok(list) => {
            let success_response = Response::FriendListResponse {
                status: "success".to_string(),
                message: "get friend list successful".to_string(),
                friends: Some(list),
            };
            let response_text = serde_json::to_string(&success_response).unwrap();
            write
                .send(tokio_tungstenite::tungstenite::Message::Text(response_text))
                .await
                .unwrap();
        }
        Err(err_msg) => {
            let fail_response = Response::FriendListResponse {
                status: "error".to_string(),
                message: err_msg,
                friends: None,
            };
            let response_text = serde_json::to_string(&fail_response).unwrap();
            write
                .send(tokio_tungstenite::tungstenite::Message::Text(response_text))
                .await
                .unwrap();
        }
    }
}

async fn get_friend_list(user_id: i32, pool: &deadpool_postgres::Pool) -> Result<Vec<FriendListItem>, String> {
    let client = pool
        .get()
        .await
        .map_err(|_| "Failed to connect to pg database".to_string())?;
    let stmt = client
        .prepare(
            "SELECT 
                u.id AS friend_id,
                u.username AS friend_name,
                m.latest_message,
                m.last_message_time,
                u.avatar_url
            FROM 
                users u
            JOIN 
                friends f ON (u.id = f.friend_id AND f.user_id = $1)
            LEFT JOIN (
                SELECT 
                    CASE 
                        WHEN m1.sender_id = $1 THEN m1.receiver_id 
                        ELSE m1.sender_id 
                    END AS friend_id,
                    m1.message AS latest_message,
                    m1.timestamp AS last_message_time
                FROM 
                    messages m1
                WHERE 
                    (m1.sender_id = $1 OR m1.receiver_id = $1)
                    AND m1.timestamp = (
                        SELECT MAX(m2.timestamp)
                        FROM messages m2
                        WHERE (m2.sender_id = m1.sender_id AND m2.receiver_id = m1.receiver_id)
                        OR (m2.sender_id = m1.receiver_id AND m2.receiver_id = m1.sender_id)
                    )
            ) m ON u.id = m.friend_id
            ORDER BY 
                m.last_message_time DESC;
                    "
        )
        .await
        .map_err(|_| "Database error".to_string())?;

    let rows = client
        .query(&stmt, &[&user_id])
        .await
        .map_err(|_| "Query failed".to_string())?;

    let friend_list_items: Vec<FriendListItem> = rows
        .iter()
        .map(|row: &tokio_postgres::Row| -> FriendListItem {
            let friend_id: i32 = row.get(0);
            let friend_name: String = row.get(1);
            let latest_message: String = row.get(2);
            let last_message_time: SystemTime = row.get(3);
            let avatar_url: Option<String> = row.get(4);
            FriendListItem {
                friend_id,
                friend_name,
                latest_message,
                last_message_time,
                avatar_url,
            }
        })
        .collect();
    Ok(friend_list_items)
}
