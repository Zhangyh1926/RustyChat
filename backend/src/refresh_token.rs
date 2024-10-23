use std::string::String;
use std::ops::Add;
use std::time::{Duration, SystemTime};
use crate::login::{ACCESS_TOKEN_EXPIRATION_TIME, REFRESH_TOKEN_EXPIRATION_TIME};
use crate::types::Response;
use crate::ws_err_utils::write_err_msg_to_ws;
use deadpool_redis::redis::cmd;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

pub async fn handle_refresh_token_request(
    user_id_for_check: i32,
    refresh_token_for_check: &str,
    write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    pool_redis: &deadpool_redis::Pool,
) {
    let mut redis_connection = pool_redis
        .get()
        .await
        .map_err(|_| "Failed to connect to redis database");
    let redis_connection = match redis_connection {
        Ok(ref mut redis_connection) => redis_connection,
        Err(msg) => {
            write_err_msg_to_ws("RefreshTokenResponse", write, msg).await;
            return;
        }
    };

    let refresh_token = match cmd("GET")
        .arg("refresh_token_".to_string() + user_id_for_check.to_string().as_str())
        .query_async::<Option<String>>(redis_connection)
        .await
    {
        Ok(token) => token,
        Err(_) => {
            write_err_msg_to_ws("RefreshTokenResponse", write, "Failed to get refresh token").await;
            return;
        }
    };

    match refresh_token {
        Some(token) => {
            if token == refresh_token_for_check {
                let new_access_token = uuid::Uuid::new_v4().to_string();
                let new_refresh_token = uuid::Uuid::new_v4().to_string();

                let _ = cmd("SET")
                    .arg("access_token_".to_string() + user_id_for_check.to_string().as_str())
                    .arg(new_access_token.clone())
                    .query_async::<()>(redis_connection)
                    .await;

                let _ = cmd("SET")
                    .arg("refresh_token_".to_string() + user_id_for_check.to_string().as_str())
                    .arg(new_refresh_token.clone())
                    .query_async::<()>(redis_connection)
                    .await;

                let _ = cmd("EXPIRE")
                    .arg("access_token_".to_string() + user_id_for_check.to_string().as_str())
                    .arg(ACCESS_TOKEN_EXPIRATION_TIME)
                    .query_async::<()>(redis_connection)
                    .await;

                let _ = cmd("EXPIRE")
                    .arg("refresh_token_".to_string() + user_id_for_check.to_string().as_str())
                    .arg(REFRESH_TOKEN_EXPIRATION_TIME)
                    .query_async::<()>(redis_connection)
                    .await;
                let response = Response::RefreshTokenResponse {
                    status: "ok".to_string(),
                    message: "Refresh token updated".to_string(),
                    refresh_token: Some(new_refresh_token),
                    access_token: Some(new_access_token),
                    access_token_expire: Some(
                        SystemTime::now().add(Duration::from_secs((ACCESS_TOKEN_EXPIRATION_TIME - 5) as u64)), // 5秒钟的缓冲时间
                    ),
                    refresh_token_expire: Some(
                        SystemTime::now()
                            .add(Duration::from_secs((REFRESH_TOKEN_EXPIRATION_TIME - 5) as u64)), // 5秒钟的缓冲时间
                    ),
                };
                let response_text = serde_json::to_string(&response).unwrap();
                write
                    .send(Message::Text(response_text))
                    .await
                    .unwrap();
            } else {
                write_err_msg_to_ws("RefreshTokenResponse", write, "Invalid refresh token").await;
            }
        },
        None => {
            write_err_msg_to_ws("RefreshTokenResponse", write, "Invalid refresh token").await;
        }
    }
}