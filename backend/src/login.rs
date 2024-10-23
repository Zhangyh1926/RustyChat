use argon2::{Argon2, PasswordHasher};
use deadpool_redis::redis::cmd;
use futures_util::{stream::SplitSink, SinkExt};
use password_hash::SaltString;
use std::ops::Add;
use std::string::String;
use std::time::{Duration, SystemTime};
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::types::Response;
use crate::ws_err_utils::write_err_msg_to_ws;

pub const ACCESS_TOKEN_EXPIRATION_TIME: u64 = 60 * 60; // 1 hour
pub const REFRESH_TOKEN_EXPIRATION_TIME: u64 = 60 * 60 * 24 * 7; // 7 days

pub async fn handle_login(
    username: &str,
    password: &str,
    write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    pool_pg: &deadpool_postgres::Pool,
    pool_redis: &deadpool_redis::Pool,
) {
    match authenticate_user(username, password, &pool_pg).await {
        Ok(id) => {
            // 生成 token, uuid
            let access_token = uuid::Uuid::new_v4().to_string();
            println!("access_token: {}", access_token);
            let refresh_token = uuid::Uuid::new_v4().to_string();
            println!("refresh_token: {}", refresh_token);
            let mut redis_connection = pool_redis
                .get()
                .await
                .map_err(|_| "Failed to connect to redis database");
            let redis_connection = match redis_connection {
                Ok(ref mut redis_connection) => redis_connection,
                Err(msg) => {
                    write_err_msg_to_ws("LoginResponse", write, msg).await;
                    return;
                }
            };
            match cmd("SET")
                .arg(&[
                    "access_token_".to_string() + id.to_string().as_str(),
                    access_token.clone(),
                    "EX".to_string(),
                    ACCESS_TOKEN_EXPIRATION_TIME.to_string(),
                ])
                .query_async::<()>(redis_connection)
                .await
            {
                Ok(_) => {}
                Err(_) => {
                    write_err_msg_to_ws("LoginResponse", write, "Failed to set access token").await;
                    return;
                }
            };
            match cmd("SET")
                .arg(&[
                    "refresh_token_".to_string() + id.to_string().as_str(),
                    refresh_token.clone(),
                    "EX".to_string(),
                    REFRESH_TOKEN_EXPIRATION_TIME.to_string(),
                ])
                .query_async::<()>(redis_connection)
                .await
            {
                Ok(_) => {}
                Err(_) => {
                    write_err_msg_to_ws("LoginResponse", write, "Failed to set access token").await;
                    return;
                }
            };

            let success_response = Response::LoginResponse {
                status: "success".to_string(),
                message: "Login successful".to_string(),
                id: Some(id),
                access_token: Some(access_token),
                refresh_token: Some(refresh_token),
                access_token_expire: Some(
                    SystemTime::now().add(Duration::from_secs((ACCESS_TOKEN_EXPIRATION_TIME - 5) as u64)), // 5秒钟的缓冲时间
                ),
                refresh_token_expire: Some(
                    SystemTime::now()
                        .add(Duration::from_secs((REFRESH_TOKEN_EXPIRATION_TIME - 5) as u64)), // 5秒钟的缓冲时间
                ),
            };
            let response_text = serde_json::to_string(&success_response).unwrap();
            write.send(Message::Text(response_text)).await.unwrap();
        }
        Err(err_msg) => {
            write_err_msg_to_ws("LoginResponse", write, &err_msg).await;
        }
    }
}

// 用户认证逻辑
async fn authenticate_user(
    username: &str,
    password: &str,
    pool: &deadpool_postgres::Pool,
) -> Result<i32, String> {
    let client = pool
        .get()
        .await
        .map_err(|_| "Failed to connect to pg database".to_string())?;
    let stmt = client
        .prepare("SELECT password_hash, salt, id FROM users WHERE username = $1")
        .await
        .map_err(|_| "Database error".to_string())?;

    let rows = client
        .query(&stmt, &[&username])
        .await
        .map_err(|_| "Query failed".to_string())?;

    if let Some(row) = rows.get(0) {
        let stored_hash: String = row.get(0);
        let salt: String = row.get(1);
        let id: i32 = row.get(2);

        // 验证密码
        let argon2 = Argon2::default();
        let salt = match SaltString::from_b64(&salt) {
            Ok(salt) => salt,
            Err(e) => return Err(e.to_string()),
        };

        let password = password.as_bytes();
        let password_hash = match argon2.hash_password(password, &salt) {
            Ok(hash) => hash.to_string(),
            Err(_) => return Err("Internal Err".to_string()),
        };
        if password_hash == stored_hash {
            Ok(id)
        } else {
            Err("Invalid credentials".to_string())
        }
    } else {
        Err("User not found".to_string())
    }
}

