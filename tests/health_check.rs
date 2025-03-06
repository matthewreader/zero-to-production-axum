use reqwest::Client;
use tokio::net::TcpListener;

#[tokio::test]
async fn test_health_check() {
    // Arrange
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(async move {
        spawn_app(listener).await.unwrap();
    });

    let client = Client::new();
    let url = format!("http://127.0.0.1:{}/health", port);

    // Act
    let response = client.get(&url).send().await.unwrap();

    // Assert
    assert_eq!(response.status(), 200);

    let json: serde_json::Value = response.json().await.unwrap();
    assert_eq!(json["status"], 200);
}

async fn spawn_app(listener: TcpListener) -> Result<(), std::io::Error> {
    zero_to_production_axum::run(listener)
}