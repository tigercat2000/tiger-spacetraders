use eyre::Result;
use std::sync::Arc;
use tiger_spacetraders::{app::App, start_ui};

#[tokio::main]
async fn main() -> Result<()> {
    let app = Arc::new(tokio::sync::Mutex::new(App::new()));

    start_ui(app).await?;
    Ok(())
}
