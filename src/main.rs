#[macro_use]
extern crate log;

mod commands;
mod constants;

use std::{convert::Infallible, error::Error, io::Write};

use axum::{routing::get, Router, Server};
use dptree::{case, deps};
use ngrok::{config::TunnelBuilder, tunnel::HttpTunnel};
use teloxide::{
    dispatching::{dialogue::InMemStorage, UpdateHandler},
    prelude::*,
    update_listeners::{webhooks, UpdateListener},
};

// use commands::Commands;
use constants::EEnvKey;

#[derive(Clone, Default)]
enum State {
    #[default]
    Start,
}

fn setup_global_logger() {
    env_logger::Builder::new()
        .filter(None, log::LevelFilter::Debug)
        .format(|buf, record| {
            writeln!(
                buf,
                "[{} - {}]\n[{}:{}] {}\n",
                chrono::Local::now().format("%d-%m-%Y %H:%M:%S%.3f"),
                record.level(),
                record.file().unwrap_or("UNKNOWN"),
                record.line().unwrap_or(0),
                record.args()
            )
        })
        .init();
}

async fn build_ngrok_listener() -> Result<HttpTunnel, Box<dyn Error>> {
    let listener = ngrok::Session::builder()
        .authtoken(EEnvKey::NGROK_AUTHTOKEN.get_env_value())
        .connect()
        .await?
        .http_endpoint()
        .domain(EEnvKey::WEBHOOK_DOMAIN.get_env_value())
        .listen()
        .await?;

    Ok(listener)
}

fn schema() -> UpdateHandler<Box<dyn Error + Send + Sync + 'static>> {
    Update::filter_message()
        .enter_dialogue::<Message, InMemStorage<State>, State>()
        .branch(
            case![State::Start].endpoint(|bot: Bot, msg: Message| async move {
                bot.send_message(msg.chat.id, "Hello World").await?;
                Ok(())
            }),
        )
}

async fn hello() -> &'static str {
    "Hello World!"
}

async fn setup_listener(
    bot: &Bot,
) -> Result<impl UpdateListener<Err = Infallible>, Box<dyn Error>> {
    let addr = EEnvKey::SOCKET_ADDR.get_env_value();
    let url = EEnvKey::WEBHOOK_DOMAIN.get_env_value();
    info!(
        "Setup webhook on:\n\tserver: {}\n\twebhook_domain: {}",
        addr, url
    );

    let (mut listener, stop_flag, bot_router) = webhooks::axum_to_router(
        bot.clone(),
        webhooks::Options::new(addr.parse()?, format!("https://{}/webhooks", url).parse()?),
    )
    .await?;
    let stop_token = listener.stop_token();
    let ngrok_listener = build_ngrok_listener().await?;

    let make_service = Router::new()
        .route("/", get(hello))
        .merge(bot_router)
        .into_make_service();
    tokio::spawn(async move {
        Server::builder(ngrok_listener)
            // Server::bind(&addr.parse().unwrap())
            .serve(make_service)
            .with_graceful_shutdown(stop_flag)
            .await
            .map_err(|e| {
                stop_token.stop();
                e
            })
            .expect("Axum server error")
    });

    Ok(listener)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenvy::dotenv()?;
    setup_global_logger();

    info!("Starting Threisd server...");

    let bot_token = EEnvKey::BOT_TOKEN.get_env_value();
    let bot = Bot::new(&bot_token);

    let listener = setup_listener(&bot).await.expect("Couldn't setup webhook");

    Dispatcher::builder(bot, schema())
        .dependencies(deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;

    Ok(())
}
