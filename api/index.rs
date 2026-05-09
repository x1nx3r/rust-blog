use rust_blog::app;
use tower::Layer;
use vercel_runtime::{Error, axum::VercelLayer, run};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let handler = VercelLayer::default().layer(app());
    run(handler).await
}
