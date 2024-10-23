use deadpool_redis::redis::cmd;
use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use crate::check_token::check_token;
use crate::types::Response;
use crate::ws_err_utils::write_err_msg_to_ws;

pub async fn handle_heartbeat_request(
    user_id_for_check: i32,
    access_token_for_check: &str,
    write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    pool_redis: &deadpool_redis::Pool,
) {
    if !check_token("HeartbeatResponse", user_id_for_check, access_token_for_check, &pool_redis, write).await {
        return;
    }
    
    // 往redis活跃用户中添加用户,生存期为60秒
    let mut redis_connection = pool_redis
        .get()
        .await
        .map_err(|_| "Failed to connect to redis database");
    let redis_connection = match redis_connection {
        Ok(ref mut redis_connection) => redis_connection,
        Err(msg) => {
            write_err_msg_to_ws("HeartbeatResponse", write, msg).await;
            return;
        }
    };

    match cmd("SET")
        .arg(&[
            "active_user_".to_string() + user_id_for_check.to_string().as_str(),
            "1".to_string(),
            "EX".to_string(),
            "60".to_string(),
        ])
        .query_async::<()>(redis_connection)
        .await
    {
        Ok(_) => {
            let success_response = Response::HeartbeatResponse {
                status: "success".to_string(),
                message: "heartbeat successful".to_string(),
            };
            let response_text = serde_json::to_string(&success_response).unwrap();
            write
                .send(Message::Text(response_text))
                .await
                .unwrap();
        }
        Err(_) => {
            write_err_msg_to_ws("HeartbeatResponse", write, "Failed to set access token").await;
            return;
        }
    };
}