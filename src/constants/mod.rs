mod env_constants;

use env_constants::EnvConstant;

lazy_static::lazy_static! {
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
        assert_ne!("".to_string(), ENV_CONSTANTS.bot_token)
    }
}
