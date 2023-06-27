use dtrack::builder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    builder().await;
    Ok(())
}
