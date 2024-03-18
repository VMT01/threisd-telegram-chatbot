use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};
use teloxide::{
    dispatching::{
        dialogue::{Dialogue, InMemStorage},
        DefaultKey, Dispatcher, UpdateHandler,
    },
    Bot,
};

use crate::states::RegistryState;

/* ========== Teloxide Types ========== */
pub type TeloxideError = Box<dyn std::error::Error + Send + Sync>;
pub type TeloxideDispatcher = Dispatcher<Bot, TeloxideError, DefaultKey>;
pub type TeloxideResponse = UpdateHandler<TeloxideError>;

/* ========== States Types ========== */
pub type RegistryDialogue = Dialogue<RegistryState, InMemStorage<RegistryState>>;

/* ========== Database Types ========== */
pub type SqlitePool = Pool<ConnectionManager<SqliteConnection>>;
