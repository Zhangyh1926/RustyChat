// src/main.rs

use std::sync::Arc;

use tokio_postgres::NoTls;

use rustychat_backend::handle_connection;

#[tokio::main]
async fn main() {
    // set up a pgsql connection pool
    let mut cfg_pg = deadpool_postgres::Config::new();
    cfg_pg.dbname = Some("rustychat".to_string());
    cfg_pg.user = Some("rustychat".to_string());
    cfg_pg.password = Some("8085050886acf353beff7".to_string());
    cfg_pg.host = Some("localhost".to_string());
    cfg_pg.port = Some(5432);

    let pool_pg = Arc::new(cfg_pg.create_pool(Some(deadpool_postgres::Runtime::Tokio1), NoTls).unwrap());
    
    let cfg_redis = deadpool_redis::Config::from_url("redis://localhost");
    let pool_redis = Arc::new(cfg_redis.create_pool(Some(deadpool_redis::Runtime::Tokio1)).unwrap());
    
    // set ws server address
    let addr = "localhost:9000".to_string();
    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .expect("Failed to bind");
    
    println!("WebSocket server listening on: {}", addr);
    
    // 接受新的 WebSocket 连接
    while let Ok((stream, peer)) = listener.accept().await {
        let pool_pg = pool_pg.clone();
        let pool_redis = pool_redis.clone();
        tokio::spawn(async move {
            let ws_stream = tokio_tungstenite::accept_async(stream)
                .await
                .expect("Failed to accept WebSocket connection");
            handle_connection(peer, ws_stream, pool_pg, pool_redis).await;
        });
    }
}
