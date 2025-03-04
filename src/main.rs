use axum::{
    routing::get,
    response::Json,
    Router,
};
use serde_json::{json, Value};
use chrono::Utc;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/health", get(health_check));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:8000")
        .await
        .expect("Failed to bind to address");

    println!("Listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service())
        .await
        .expect("Failed to run server");
}

async fn health_check() -> Json<Value> {
    let dtm = Utc::now();
    let response: Value = json!({
        "status": 200,
        "time": dtm
    });
    Json(response)
}
