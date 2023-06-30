#[tokio::main]
async fn main() -> Result<(), dtrack::error::DtrackError> {
    if let Err(e) = dtrack::builder().await {
        eprintln!("程序运行出错: {}", e);
    }
    Ok(())
}
