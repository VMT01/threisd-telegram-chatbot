mod configs;
mod constants;
mod handlers;
mod models;
mod schema;
mod states;
mod threisd;
mod types;

#[tokio::main]
async fn main() {
    threisd::start().await;
}
