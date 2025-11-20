#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app = pm5::App::new()?;
    app.run().await?;
    Ok(())
}
