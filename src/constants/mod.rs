mod env_constants;

use lazy_static::lazy_static;

use self::env_constants::EnvConstant;

lazy_static! {
    pub static ref ENV_CONSTANTS: EnvConstant = EnvConstant::init();
}
