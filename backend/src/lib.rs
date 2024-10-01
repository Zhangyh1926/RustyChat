// src/lib.rs

mod friend_list;
mod login;
mod message_list;
mod message_push;
mod types;

use anyhow::Result;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHasher, SaltString},
    Argon2,
};
use deadpool_postgres::Pool;
use futures_util::{stream::StreamExt, SinkExt};
use std::sync::Arc;
use types::{Request, Response};

pub async fn handle_connection(
    peer: std::net::SocketAddr,
    ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    pool: Arc<Pool>,
) {
    println!("New WebSocket connection from: {}", peer);

    let (mut write, mut read) = ws_stream.split();

    while let Some(msg) = read.next().await {
        if let Ok(msg) = msg {
            if msg.is_text() {
                let msg_text = msg.to_text().unwrap();
                println!("Received message: {}", msg_text);

                let request: Request = match serde_json::from_str(msg_text) {
                    Ok(req) => req,
                    Err(_) => {
                        let error_response = Response::LoginResponse {
                            status: "error".to_string(),
                            message: "Invalid JSON format".to_string(),
                            id: None,
                        };
                        let response_text = serde_json::to_string(&error_response).unwrap();
                        println!("Invalid JSON format");
                        write
                            .send(tokio_tungstenite::tungstenite::Message::Text(response_text))
                            .await
                            .unwrap();
                        continue;
                    }
                };

                match request {
                    Request::LoginRequest {
                        username: _,
                        password: _,
                    } => {
                        login::handle_login(&request, &mut write, &pool).await;
                    }
                    Request::FriendListRequest { userid } => {
                        friend_list::handle_friendlist_request(userid, &mut write, &pool).await;
                    }
                    Request::MessageListRequest {
                        my_userid,
                        other_userid,
                    } => {
                        message_list::handle_message_list_request(
                            my_userid,
                            other_userid,
                            &mut write,
                            &pool,
                        )
                        .await;
                    }

                    _ => {
                        let error_response = Response::LoginResponse {
                            status: "error".to_string(),
                            message: "Invalid message type".to_string(),
                            id: None,
                        };
                        let response_text = serde_json::to_string(&error_response).unwrap();
                        write
                            .send(tokio_tungstenite::tungstenite::Message::Text(response_text))
                            .await
                            .unwrap();
                    }
                }
            }
        }
    }
}

pub async fn register_user(username: &str, password: &str, pool: &Pool) -> Result<()> {
    let client = pool.get().await.unwrap();
    let stmt = client
        .prepare("INSERT INTO users (username, password_hash, salt) VALUES ($1, $2, $3)")
        .await?;

    // 使用 argon2 加密密码
    let salt = SaltString::generate(&mut OsRng);

    // Argon2 with default params (Argon2id v19)
    let argon2 = Argon2::default();

    // Hash password to PHC string ($argon2id$v=19$...)
    let password = password.as_bytes();
    let password_hash = match argon2.hash_password(password, &salt) {
        Ok(hash) => hash.to_string(),
        Err(e) => return Err(anyhow::anyhow!(e)),
    };
    let salt: String = salt.to_string();

    client
        .execute(&stmt, &[&username, &password_hash, &salt])
        .await?;

    Ok(())
}
