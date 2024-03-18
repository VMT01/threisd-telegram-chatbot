use std::env;

use dotenv::dotenv;

enum EEnvKey {
    BotToken,
    NgrokToken,
    SocketAddr,
    WebhookDomain,
    DatabaseUrl,
}

impl From<EEnvKey> for String {
    fn from(value: EEnvKey) -> Self {
        match value {
            EEnvKey::BotToken => {
                env::var("BOT_TOKEN").unwrap_or_else(|_| panic!("BOT_TOKEN must be set!"))
            }
            EEnvKey::NgrokToken => env::var("NGROK_AUTHTOKEN")
                .unwrap_or_else(|_| panic!("NGROK_AUTHTOKEN must be set!")),
            EEnvKey::SocketAddr => env::var("SOCKET_ADDR").unwrap_or("0.0.0.0:8443".into()),
            EEnvKey::WebhookDomain => {
                env::var("WEBHOOK_DOMAIN").unwrap_or("bunny-right-beetle.ngrok-free.app".into())
            }
            EEnvKey::DatabaseUrl => {
                env::var("DATABASE_URL").unwrap_or_else(|_| panic!("DATABASE_URL must be set"))
            }
        }
    }
}

pub struct EnvConstant {
    /// Telegram bot api token
    pub bot_token: String,

    /// Ngrok authentication token
    pub ngrok_token: String,

    /// Server address for listen event from Telegram
    pub socket_addr: String,

    /// Tunel address for Telegram send event
    pub webhook_domain: String,

    /// Database url
    pub database_url: String,
}

impl EnvConstant {
    pub fn init() -> Self {
        dotenv().unwrap_or_else(|_| panic!("Error while loading .env"));

        Self {
            bot_token: EEnvKey::BotToken.into(),
            ngrok_token: EEnvKey::NgrokToken.into(),
            socket_addr: EEnvKey::SocketAddr.into(),
            webhook_domain: EEnvKey::WebhookDomain.into(),
            database_url: EEnvKey::DatabaseUrl.into(),
        }
    }
}
