// src/lib.rs

mod friend_list;
mod login;
mod message_list;
mod message_push;
mod types;
mod register;
mod friend_center;
mod heartbeat;
mod check_token;
mod ws_err_utils;
mod refresh_token;

use crate::register::handle_register_request;
use futures_util::{stream::StreamExt, SinkExt};
use std::sync::Arc;
use types::{Request, Response};

pub async fn handle_connection(
    peer: std::net::SocketAddr,
    ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
    pool_pg: Arc<deadpool_postgres::Pool>,
    pool_redis: Arc<deadpool_redis::Pool>,
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
                        let error_response = Response::GenericResponse {
                            status: "error".to_string(),
                            message: "Invalid JSON format".to_string(),
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
                        username, password, ..
                    } => {
                        login::handle_login(&username, &password, &mut write, &pool_pg, &pool_redis).await;
                    }
                    Request::FriendListRequest { user_id_for_check, access_token_for_check } => {
                        friend_list::handle_friendlist_request(user_id_for_check, &access_token_for_check, &mut write, &pool_pg, &pool_redis).await;
                    }
                    Request::MessageListRequest {
                        other_userid, user_id_for_check, access_token_for_check
                    } => {
                        message_list::handle_message_list_request(
                            user_id_for_check,
                            &access_token_for_check,
                            other_userid,
                            &mut write,
                            &pool_pg,
                            &pool_redis,
                        )
                        .await;
                    }
                    Request::MessagePushRequest {
                        receiver_id,
                        message,
                        user_id_for_check,
                        access_token_for_check,
                    } => {
                        message_push::handle_message_push_request(
                            user_id_for_check,
                            &access_token_for_check,
                            receiver_id,
                            &message,
                            &mut write,
                            &pool_pg,
                            &pool_redis,
                        )
                        .await;
                    }
                    Request::RegisterRequest {
                        username,
                        password,..
                    } => {
                        handle_register_request(&username, &password, &mut write, &pool_pg).await;
                    }
                    Request::FriendCenterRequest { user_id_for_check, access_token_for_check} => {
                        friend_center::handle_friend_center_request(
                            user_id_for_check,
                            &access_token_for_check,
                            &mut write,
                            &pool_pg,
                            &pool_redis,
                        )
                        .await;
                    }
                    Request::HeartbeatRequest { user_id_for_check, access_token_for_check } => {
                        heartbeat::handle_heartbeat_request(
                            user_id_for_check,
                            &access_token_for_check,
                            &mut write,
                            &pool_redis,
                        )
                        .await;
                    }   
                }
            }
        }
    }
}

