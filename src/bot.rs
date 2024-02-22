use anyhow::Result;
use axum::{Router, Server};
use log::{error, info, warn, LevelFilter};
use log4rs::{
    append::{
        console::{ConsoleAppender, Target},
        file::FileAppender,
    },
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Config,
};
use ngrok::{config::TunnelBuilder, Session};
use std::convert::Infallible;
use teloxide::{
    dispatching::Dispatcher,
    prelude::*,
    update_listeners::{webhooks, UpdateListener},
};

use crate::{constants::ENV_CONSTANTS, handlers::build_handlers, types::TeloxideDispatcher};

/// Thiết lập logger
fn setup_logger() -> Result<()> {
    let pattern = PatternEncoder::new("[{d(%d-%m-%YT%H:%M:%S%.3f)} {h({l})}]\n[{f}:{L}] {m}\n\n");

    let stdout: ConsoleAppender = ConsoleAppender::builder()
        .target(Target::Stdout)
        .encoder(Box::new(pattern.clone()))
        .build();

    let logfile: FileAppender = FileAppender::builder()
        .encoder(Box::new(pattern))
        .append(false)
        .build(".log")?;

    let config: Config = Config::builder()
        .appender(
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
                .build("stdout", Box::new(stdout)),
        )
        .appender(Appender::builder().build("logfile", Box::new(logfile)))
        .build(
            Root::builder()
                .appender("stdout")
                .appender("logfile")
                .build(LevelFilter::Debug),
        )?;

    log4rs::init_config(config)?;

    Ok(())
}

/// Thiết lập webhook trên nền Ngrok
async fn setup_listener(bot: &Bot) -> Result<impl UpdateListener<Err = Infallible>> {
    info!(
        "Setup webhook on: \n - Socket address: {}\n - Webhook domain: {}",
        ENV_CONSTANTS.socket_addr, ENV_CONSTANTS.webhook_domain
    );

    let webhook_url = format!(
        "https://{}/webhook-bot{}",
        ENV_CONSTANTS.webhook_domain, ENV_CONSTANTS.bot_token
    )
    .parse()
    .unwrap_or_else(|e| panic!("Error while parsing webhook url: {:?}", e));
    let options = webhooks::Options::new(ENV_CONSTANTS.socket_addr, webhook_url);
    let (mut listener, stop_flag, bot_router) =
        webhooks::axum_to_router(bot.clone(), options).await?;
    let stop_token = listener.stop_token();

    info!("Setup Ngrok listener");
    let ngrok_listener = Session::builder()
        .authtoken(&ENV_CONSTANTS.ngrok_auth_token)
        .connect()
        .await?
        .http_endpoint()
        .domain(&ENV_CONSTANTS.webhook_domain)
        .listen()
        .await?;

    info!("Run webhook server");
    let make_service = Router::new()
        .merge(bot_router)
        .fallback(|| async { warn!("Axum: 404") })
        .into_make_service();
    tokio::spawn(async move {
        Server::builder(ngrok_listener)
            .serve(make_service)
            .with_graceful_shutdown(stop_flag)
            .await
            .map_err(|e| {
                stop_token.stop();
                error!("{:?}", e);
                e
            })
            .expect("Axum server error!")
    });

    Ok(listener)
}

/// Truyền phát dữ liệu qua webhook
fn build_dispatcher(bot: Bot) -> TeloxideDispatcher {
    Dispatcher::builder(bot, build_handlers())
        .dependencies(dptree::deps![])
        .enable_ctrlc_handler()
        .default_handler(|upd| async move {
            warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .build()
}

/// Khởi chạy bot
pub async fn start() -> Result<()> {
    setup_logger()?;
    info!("Starting Threisd server...");

    let bot = Bot::new(&ENV_CONSTANTS.bot_token);
    let listener = setup_listener(&bot).await?;
    let mut dispatcher = build_dispatcher(bot);

    dispatcher
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;

    Ok(())
}
