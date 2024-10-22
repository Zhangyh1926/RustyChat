use crate::types::Response;
use anyhow::anyhow;
use anyhow::Result;
use argon2::Argon2;
use deadpool_postgres::Pool;
use futures_util::stream::SplitSink;
use futures_util::SinkExt;
use password_hash::rand_core::OsRng;
use password_hash::{PasswordHasher, SaltString};
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;

pub async fn handle_register_request(
    username: &str,
    password: &str,
    write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    pool: &Pool,
) {
    match register_user(username, password, &pool).await {
        Ok(_) => {
            let success_response = Response::RegisterResponse {
                status: "success".to_string(),
                message: "successfully registered".to_string(),
                id: None,
            };
            let response_text = serde_json::to_string(&success_response).unwrap();
            write
                .send(tokio_tungstenite::tungstenite::Message::Text(response_text))
                .await
                .unwrap();
        }
        Err(err_msg) => {
            let fail_response = Response::MessageListResponse {
                status: "error".to_string(),
                message: err_msg.to_string(),
                messages: None,
            };
            let response_text = serde_json::to_string(&fail_response).unwrap();
            write
                .send(tokio_tungstenite::tungstenite::Message::Text(response_text))
                .await
                .unwrap();
        }
    }
}

pub async fn register_user(username: &str, password: &str, pool: &Pool) -> Result<()> {
    let client = pool.get().await?;
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
        Err(e) => return Err(anyhow!("Hash password failed: {}", e)),
    };
    let salt: String = salt.to_string();

    client
        .execute(&stmt, &[&username, &password_hash, &salt])
        .await
        .map_err(|_| anyhow!("There have already user with same name!"))?;

    Ok(())
}
