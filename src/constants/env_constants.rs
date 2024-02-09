use core::panic;
use std::{env, net::SocketAddr};

#[allow(non_camel_case_types)]
enum EEnvKey {
    BOT_TOKEN,
    NGROK_AUTHTOKEN,
    WEBHOOK_DOMAIN,
    SOCKET_ADDR,
}

impl EEnvKey {
    fn to_str(&self) -> &str {
        match self {
            EEnvKey::BOT_TOKEN => "BOT_TOKEN",
            EEnvKey::NGROK_AUTHTOKEN => "NGROK_AUTHTOKEN",
            EEnvKey::WEBHOOK_DOMAIN => "WEBHOOK_DOMAIN",
            EEnvKey::SOCKET_ADDR => "SOCKET_ADDR",
        }
    }
}

pub struct EnvConstant {
    pub bot_token: String,
    pub ngrok_auth_token: String,
    pub socket_addr: SocketAddr,
    pub webhook_domain: String,
}

impl EnvConstant {
    pub fn init() -> Self {
        let bot_token: String = env::var(EEnvKey::BOT_TOKEN.to_str())
            .unwrap_or_else(|_| panic!("BOT_TOKEN must be set!"));

        let ngrok_auth_token: String = env::var(EEnvKey::NGROK_AUTHTOKEN.to_str())
            .unwrap_or_else(|_| panic!("NGROK_AUTHTOKEN must be set!"));

        let socket_addr: SocketAddr = env::var(EEnvKey::SOCKET_ADDR.to_str())
            .unwrap_or("0.0.0.0:8443".to_string())
            .parse()
            .unwrap_or_else(|e| panic!("Error while parsing socket address: {:?}", e));

        let webhook_domain: String = env::var(EEnvKey::WEBHOOK_DOMAIN.to_str())
            .unwrap_or("bunny-right-beetle.ngrok-free.app".to_string());

        Self {
            bot_token,
            ngrok_auth_token,
            socket_addr,
            webhook_domain,
        }
    }
}
