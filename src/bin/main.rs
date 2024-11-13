#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = pm5::App {};
    app.run().await
}