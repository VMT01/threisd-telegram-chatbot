use std::convert::Infallible;

use axum::Router;
use log::{info, warn, LevelFilter};
use log4rs::{
    append::{console::ConsoleAppender, file::FileAppender},
    config::{Appender, Root},
    encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
    Config,
};
use ngrok::{config::TunnelBuilder, Session};
use teloxide::{
    dispatching::dialogue::InMemStorage,
    prelude::*,
    update_listeners::{webhooks, UpdateListener},
};

use crate::{
    constants::ENV_CONSTANTS,
    handlers::build_handlers,
    states::RegistryState,
    types::{TeloxideDispatcher, TeloxideResponse},
};

/// Setup logger for STDOUT and FILE
/// - STDOUT: Only print log from `Info` level
/// - FILE: Only print log from `Debug` level
fn setup_logger() {
    let encoder = "[{d(%d-%m-%YT%H:%M:%S%.3f)} {h({l})}]\n[{f}:{L}]\n{m}\n\n";

    let stdout = Appender::builder()
        .filter(Box::new(ThresholdFilter::new(LevelFilter::Info)))
        .build(
            "stdout",
            Box::new(
                ConsoleAppender::builder()
                    .encoder(Box::new(PatternEncoder::new(encoder)))
                    .build(),
            ),
        );

    let logfile = Appender::builder().build(
        "logfile",
        Box::new(
            FileAppender::builder()
                .encoder(Box::new(PatternEncoder::new(encoder)))
                .append(false)
                .build(".log")
                .unwrap_or_else(|err| panic!("Error while building log file: {}", err)),
        ),
    );

    let config = Config::builder()
        .appenders([stdout, logfile])
        .build(
            Root::builder()
                .appenders(["stdout", "logfile"])
                .build(LevelFilter::Debug),
        )
        .unwrap_or_else(|err| panic!("Error while building log4rs config: {}", err));

    log4rs::init_config(config)
        .unwrap_or_else(|err| panic!("Error while init log4rs config: {}", err));
}

async fn setup_listener(bot: Bot) -> impl UpdateListener<Err = Infallible> {
    info!(
        "Setup webhook:\n - Server address: {}\n - Webhook domain: {}",
        ENV_CONSTANTS.socket_addr, ENV_CONSTANTS.webhook_domain
    );

    let (mut listener, stop_flag, bot_router) = webhooks::axum_to_router(
        bot,
        webhooks::Options::new(
            ENV_CONSTANTS
                .socket_addr
                .parse()
                .unwrap_or_else(|err| panic!("Error while parsing socket address: {}", err)),
            format!(
                "https://{}/webhook-bot{}",
                ENV_CONSTANTS.webhook_domain, ENV_CONSTANTS.bot_token,
            )
            .parse()
            .unwrap_or_else(|err| panic!("Error while parsing webhook domain: {}", err)),
        ),
    )
    .await
    .unwrap_or_else(|err| panic!("Error while initializing server listener: {}", err));
    let stop_token = listener.stop_token();

    info!("Setup Ngrok listener");
    let ngrok_listener = Session::builder()
        .authtoken(&ENV_CONSTANTS.ngrok_token)
        .connect()
        .await
        .unwrap_or_else(|err| panic!("Error while trying to connect Ngrok server: {}", err))
        .http_endpoint()
        .domain(&ENV_CONSTANTS.webhook_domain)
        .listen()
        .await
        .unwrap_or_else(|err| panic!("Error while trying to create tunnel: {}", err));

    info!("Run webhook server");
    let make_service = Router::new()
        .merge(bot_router)
        .fallback(|| async { warn!("Axum: 404") })
        .into_make_service();
    tokio::spawn(async move {
        axum::Server::builder(ngrok_listener)
            .serve(make_service)
            .with_graceful_shutdown(stop_flag)
            .await
            .unwrap_or_else(|err| {
                stop_token.stop();
                panic!("Error while starting server: {}", err);
            })
    });

    listener
}

async fn build_dispatcher(bot: Bot, handler: TeloxideResponse) -> TeloxideDispatcher {
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<RegistryState>::new()])
        .enable_ctrlc_handler()
        .default_handler(|upd| async move {
            warn!("Unhandled update: {:?}", upd);
        })
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .build()
}

pub async fn start() {
    setup_logger();
    info!("Starting Threisd server...");

    let bot = Bot::new(&ENV_CONSTANTS.bot_token);
    let listener = setup_listener(bot.clone()).await;
    let handlers = build_handlers();
    let mut dispatcher = build_dispatcher(bot, handlers).await;

    dispatcher
        .dispatch_with_listener(
            listener,
            LoggingErrorHandler::with_custom_text("An error from the update listener"),
        )
        .await;
}
