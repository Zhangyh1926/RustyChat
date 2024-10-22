// src/message_list.rs
use std::time::SystemTime;

use futures_util::{stream::SplitSink, SinkExt};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};
use crate::check_token::check_token;
use crate::types::{MessageListItem, Response};

pub async fn handle_message_list_request(
    user_id: i32,
    access_token_for_check: &str,
    another_user_id: i32,
    write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    pool_pg: &deadpool_postgres::Pool,
    pool_redis: &deadpool_redis::Pool,
) {
    if !check_token(user_id, access_token_for_check, &pool_redis, write).await {
        return;
    }
    match get_message_list(user_id, another_user_id, &pool_pg).await {
        Ok(list) => {
            let success_response = Response::MessageListResponse { 
                status: "success".to_string(),
                message: "get message list successful".to_string(),
                messages: Some(list),
            };
            let response_text = serde_json::to_string(&success_response).unwrap();
            write
                .send(Message::Text(response_text))
                .await
                .unwrap();
        }
        Err(err_msg) => {
            let fail_response = Response::MessageListResponse {
                status: "error".to_string(),
                message: err_msg,
                messages: None,
            };
            let response_text = serde_json::to_string(&fail_response).unwrap();
            write
                .send(Message::Text(response_text))
                .await
                .unwrap();
        }
    }
}

async fn get_message_list(user_id: i32, another_user_id: i32, pool: &deadpool_postgres::Pool) -> Result<Vec<MessageListItem>, String> {
    let client = pool
        .get()
        .await
        .map_err(|_| "Failed to connect to database".to_string())?;
    let stmt = client
        .prepare(
            "SELECT 
                id AS message_id,
                sender_id,
                receiver_id,
                message,
                timestamp
            FROM 
                messages m
            WHERE
                (sender_id = $1 AND receiver_id = $2) OR (sender_id = $2 AND receiver_id = $1)
            ORDER BY 
                timestamp ASC;
                    "
        )
        .await
        .map_err(|_| "Database error".to_string())?;

    let rows = client
        .query(&stmt, &[&user_id, &another_user_id])
        .await
        .map_err(|_| "Query failed".to_string())?;

    let message_list_items: Vec<MessageListItem> = rows
        .iter()
        .map(|row: &tokio_postgres::Row| -> MessageListItem {
            let message_id: i32 = row.get(0);
            let sender_id: i32 = row.get(1);
            let receiver_id: i32 = row.get(2);
            let message: String = row.get(3);
            let send_time: SystemTime = row.get(4);
            MessageListItem {
                message_id,
                sender_id,
                receiver_id,
                message,
                send_time,
            }
        })
        .collect();
    Ok(message_list_items)
}