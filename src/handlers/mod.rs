use teloxide::{dispatching::UpdateHandler, prelude::*, RequestError};

mod authen;
mod test_command;

// pub use authen::AuthenState;

pub fn build_handlers() -> UpdateHandler<RequestError> {
    dptree::entry()
        /* Authen handler */
        .branch(authen::build_handler())
        /* Test command handler */
        .branch(test_command::build_handler())
}
