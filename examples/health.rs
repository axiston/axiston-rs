use axiston::{Client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::default();
    client.health().await?;

    Ok(())
}
