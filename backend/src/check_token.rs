use deadpool_redis::redis::AsyncCommands;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use crate::ws_err_utils::write_err_msg_to_ws;

pub async fn check_token(
    user_id_for_check: i32,
    access_token_for_check: &str,
    pool_redis: &deadpool_redis::Pool,
    write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
) -> bool {
    let mut redis_connection = pool_redis
        .get()
        .await
        .map_err(|_| "Failed to connect to database".to_string());
    let redis_connection = match redis_connection {
        Ok(ref mut redis_connection) => redis_connection,
        Err(_) => {
            write_err_msg_to_ws(write, "Failed to connect to database").await;
            return false;
        }
    };
    let access_token: String = redis_connection.get("access_token_".to_string() + user_id_for_check.to_string().as_str()).await.unwrap();

    if access_token == access_token_for_check {
        true
    } else {
        write_err_msg_to_ws(write, "Invalid access token").await;
        false
    }
}