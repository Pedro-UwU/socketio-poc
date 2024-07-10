use axum::routing::get;
use socketioxide::{
    extract::{Data, SocketRef},
    SocketIo,
};
use tokio::net::TcpListener;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;
use tracing::info;
use tracing_subscriber::FmtSubscriber;

#[derive(Debug, serde::Deserialize)]
struct MessageIn {
    room: String,
    text: String,
}

#[derive(Debug, serde::Serialize)]
struct MessageOut {
    text: String,
    user: String,
    date: chrono::DateTime<chrono::Utc>,
}

async fn on_connect(socket: SocketRef) {
    info!("socket connected: {}", socket.id);

    socket.on("join", |socket: SocketRef, Data::<String>(room)| {
        info!("received join {:?}", room);
        let _ = socket.leave_all();
        let _ = socket.join(room);
    });

    socket.on("message", |socket: SocketRef, Data::<MessageIn>(data)| {
        info!("Recieved message: {:?}", data);

        let response = MessageOut {
            text: data.text,
            user: format!("anon-{}", socket.id),
            date: chrono::Utc::now(),
        };

        let _ = socket.within(data.room).emit("message", response);
    });

}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let _ = tracing::subscriber::set_global_default(FmtSubscriber::default());

    let (layer, io) = SocketIo::new_layer();

    io.ns("/", on_connect);

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World! " }))
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    info!("Starting server");

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    axum::serve(listener, app.into_make_service()).await?;
    Ok(())
}
