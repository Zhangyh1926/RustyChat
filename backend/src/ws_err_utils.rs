use futures_util::SinkExt;
use futures_util::stream::SplitSink;
use tokio::net::TcpStream;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::WebSocketStream;
use crate::types::Response;

pub async fn write_err_msg_to_ws(
    write: &mut SplitSink<WebSocketStream<TcpStream>, Message>,
    err_msg: &str,
) {
    let fail_response = Response::GenericResponse {
        status: "error".to_string(),
        message: err_msg.to_string(),
    };
    let response_text = serde_json::to_string(&fail_response).unwrap();
    write.send(Message::Text(response_text)).await.unwrap();
}