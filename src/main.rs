mod bot;
mod constants;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;
    bot::start().await?;
    Ok(())
}
