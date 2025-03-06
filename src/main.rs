use zero_to_production_axum::run;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8000").await?;
    run(listener)?;

    tokio::signal::ctrl_c().await?;
    Ok(())
}
