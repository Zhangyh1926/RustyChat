// src/main.rs

use std::sync::Arc;

use deadpool_postgres::{Config, Runtime};
use tokio_postgres::NoTls;

use rustychat_backend::handle_connection;

#[tokio::main]
async fn main() {
    // set up a pgsql connection pool
    let mut cfg = Config::new();
    cfg.dbname = Some("rustychat".to_string());
    cfg.user = Some("rustychat".to_string());
    cfg.password = Some("8085050886acf353beff7".to_string());
    cfg.host = Some("localhost".to_string());
    cfg.port = Some(5432);

    let pool = Arc::new(cfg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap());

    // match register_user("test", "123", &pool).await {
    //     Ok(_) => println!("ok"),
    //     Err(e) => println!("{}", e.to_string())
    // };
    // set ws server address
    let addr = "localhost:9000".to_string();
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind");

    println!("WebSocket server listening on: {}", addr);

    // 接受新的 WebSocket 连接
    while let Ok((stream, peer)) = listener.accept().await {
        let pool = pool.clone();
        tokio::spawn(async move {
            let ws_stream = tokio_tungstenite::accept_async(stream)
                .await
                .expect("Failed to accept WebSocket connection");
            handle_connection(peer, ws_stream, pool).await;
        });
    }
}
