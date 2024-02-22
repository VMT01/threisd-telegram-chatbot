mod env_constants;

use lazy_static::lazy_static;

use self::env_constants::EnvConstant;

lazy_static! {
    pub static ref ENV_CONSTANTS: EnvConstant = EnvConstant::init();
}

#[cfg(test)]
mod tests {
    use super::{EnvConstant, ENV_CONSTANTS};

    #[test]
    fn get_env() {
        let env_constants = EnvConstant::init();
        assert_ne!("".to_string(), env_constants.bot_token);
    }

    #[test]
    fn get_lazy_env() {
        let env_constants = &ENV_CONSTANTS;
        assert_ne!("".to_string(), env_constants.database_url);
    }
}
