use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    SqliteConnection,
};
use teloxide::{
    dispatching::{DefaultKey, Dispatcher},
    Bot, RequestError,
};

/* ========== Teloxide Types ========== */
pub type TeloxideDispatcher = Dispatcher<Bot, RequestError, DefaultKey>;
// pub type HandlerResult = Result<(), Error>;

/* ========== Database Types ========== */
pub type SqliteConnectionManager = ConnectionManager<SqliteConnection>;
pub type SqlitePool = Pool<SqliteConnectionManager>;
pub type SqlitePooledConnection = PooledConnection<SqliteConnectionManager>;
