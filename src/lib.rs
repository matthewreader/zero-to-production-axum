use axum::{routing::get, Json, Router};
use axum::http::StatusCode;
use tokio::net::TcpListener;
use axum::response::IntoResponse;
use chrono::Utc;
use serde_json::{json, Value};
use tracing::{info, error};
use tracing_subscriber;

pub fn run(listener: TcpListener) -> Result<(), std::io::Error> {
    tracing_subscriber::fmt::init();

    let addr = listener.local_addr().expect("could not get listener addr");
    info!("Server is running on: {}", addr);

    let app = Router::new()
        .route("/health", get(health_check));

    let server = axum::serve(listener, app.into_make_service());

    tokio::spawn(async move {
        if let Err(e) = server.await {
            error!("server error: {}", e);
        }
    });

    Ok(())
}
async fn health_check() -> impl IntoResponse {
    let dtm = Utc::now();
    let response: Value = json!({
        "status": 200,
        "time": dtm
    });
    (StatusCode::OK, Json(response))
}