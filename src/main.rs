use std::sync::Arc;

use holytea_api::{api::router::{create_router, AppState}, infrastructure::database::establish_connection};
use dotenv::dotenv;
use tokio::net::TcpListener;
#[tokio::main]
async fn main() {
    dotenv().ok();
    //Init Connection
    let pool = establish_connection().await;
    let shared_state = Arc::new(AppState{db:pool});

    // Create the router and attach the state
    let app = create_router(shared_state);
    println!("✅ Server started successfully at localhost:8080");
    let listener = TcpListener::bind("localhost:8080").await.unwrap();
    let local_addr = listener.local_addr().unwrap();
    let url = if local_addr.is_ipv6() {
        // Convert the IPv6 address to its IPv4 equivalent (if possible)
        format!("http://localhost:8080") // or use local_addr.ip().to_string() for IPv4 specifically
    } else {
        format!("http://127.0.0.0:8080")
    };
    
    println!("ini url {}",url);
    axum::serve(listener,app).await.unwrap();
}
