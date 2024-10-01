use argon2::{Argon2, PasswordHasher};
use deadpool_postgres::Pool;
use futures_util::{stream::SplitSink, SinkExt };
use password_hash::SaltString;
use tokio::net::TcpStream;
use tokio_tungstenite::{tungstenite::Message, WebSocketStream};

use crate::types::{Request, Response};

pub async fn handle_login(
    login_request: &crate::Request,
    write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    pool: &Pool,
) {
    match authenticate_user(&login_request, &pool).await {
        Ok(token) => {
            let success_response = Response::LoginResponse {
                status: "success".to_string(),
                message: "Login successful".to_string(),
                id: Some(token),
            };
            let response_text = serde_json::to_string(&success_response).unwrap();
            write
                .send(tokio_tungstenite::tungstenite::Message::Text(response_text))
                .await
                .unwrap();
        }
        Err(err_msg) => {
            let fail_response = Response::LoginResponse {
                status: "error".to_string(),
                message: err_msg,
                id: None,
            };
            let response_text = serde_json::to_string(&fail_response).unwrap();
            write
                .send(tokio_tungstenite::tungstenite::Message::Text(response_text))
                .await
                .unwrap();
        }
    }
}

// 用户认证逻辑
async fn authenticate_user(login_request: &Request, pool: &Pool) -> Result<i32, String> {
    match &login_request {
        Request::LoginRequest { username, password } => {
            let client = pool
                .get()
                .await
                .map_err(|_| "Failed to connect to database".to_string())?;
            let stmt = client
                .prepare("SELECT password_hash, salt, id FROM users WHERE username = $1")
                .await
                .map_err(|_| "Database error".to_string())?;

            let rows = client
                .query(&stmt, &[username])
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
                    return Ok(id);
                } else {
                    return Err("Invalid credentials".to_string());
                }
            } else {
                return Err("User not found".to_string());
            }
        }
        _ => return Err("Invalid message type".to_string()),
    }
}
