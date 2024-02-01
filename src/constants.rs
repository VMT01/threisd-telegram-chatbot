#![allow(non_camel_case_types)]

use std::env;
pub enum EEnvKey {
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

    pub fn get_env_value(&self) -> String {
        let key = self.to_str();
        env::var(key).unwrap_or_else(|_| panic!("{} must be set!", key))
    }
}
