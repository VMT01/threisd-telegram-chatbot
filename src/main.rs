mod bot;
mod configs;
mod constants;
mod handlers;
mod models;
mod schema;
mod types;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    bot::start().await?;
    Ok(())
}
