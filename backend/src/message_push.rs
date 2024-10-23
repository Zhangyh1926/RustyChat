use std::time::SystemTime;
use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use crate::check_token::check_token;
use crate::types::Response;

pub async fn handle_message_push_request(
    sender_id: i32,
    access_token_for_check: &str,
    receiver_id: i32,
    message: &str,
    write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    pool_pg: &deadpool_postgres::Pool,
    pool_redis: &deadpool_redis::Pool,
) {
    if !check_token("MessagePushResponse", sender_id, access_token_for_check, &pool_redis, write).await {
        return;
    }
    match push_messages(sender_id, receiver_id, message, &pool_pg).await {
        Ok((_, _)) => {
            let success_response = Response::MessagePushResponse {
                status: "success".to_string(),
                message: "push message successful".to_string(),
            };
            let response_text = serde_json::to_string(&success_response).unwrap();
            write
                .send(Message::Text(response_text))
                .await
                .unwrap();
        }
        Err(err_msg) => {
            let fail_response = Response::MessagePushResponse {
                status: "error".to_string(),
                message: err_msg,
            };
            let response_text = serde_json::to_string(&fail_response).unwrap();
            write
                .send(Message::Text(response_text))
                .await
                .unwrap();
        }
    }
}

async fn push_messages(sender_id: i32, receiver_id: i32, message: &str, pool: &deadpool_postgres::Pool) -> Result<(SystemTime, i32), String> {
    let client = pool
        .get()
        .await
        .map_err(|_| "Failed to connect to pg database".to_string())?;
    let stmt = client
        .prepare(
            "INSERT INTO messages(sender_id, receiver_id, message) VALUES($1, $2, $3) RETURNING timestamp, id;"
        )
        .await
        .map_err(|_| "Database error".to_string())?;

    let rows = client
        .query(&stmt, &[&sender_id, &receiver_id, &message])
        .await
        .map_err(|_| "Query failed".to_string())?;
    
    let message_push_item = (rows[0].get(0), rows[0].get(1));
    Ok(message_push_item)
}
