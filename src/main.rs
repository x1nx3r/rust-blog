use rust_blog::app;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = app();
    let listener = TcpListener::bind("0.0.0.0:3789").await.unwrap();
    println!("Listening on http://localhost:3789");
    axum::serve(listener, app).await.unwrap();
}
