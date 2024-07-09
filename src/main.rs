use axum::routing::get;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing::subscriber::set_global_default(FmtSubscriber::default());
    
    let app = axum::Router::new().route("/", get(| | async { "Hello, World! "}));

    info!("Starting server");

    axum::Server::bind(&"127.0.0.1:3000".parse().unwrap())
        .serve(app.into_make_service())
        .await?;
    Ok(())
}
