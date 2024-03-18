mod callback_handlers;
mod inline_query_handlers;
mod message_handlers;

use teloxide::{dispatching::dialogue::InMemStorage, prelude::*};

use crate::{states::RegistryState, types::TeloxideResponse};

use self::{
    callback_handlers::{CallbackFilter, CallbackHandler},
    inline_query_handlers::InlineQueryHandler,
    message_handlers::{MessageFilter, MessageHandler},
};

pub fn build_handlers() -> TeloxideResponse {
    dptree::entry()
        .branch(
            Update::filter_message()
                .branch(
                    dptree::filter(MessageFilter::filter_valid_user)
                        .endpoint(MessageHandler::authen_user_handler),
                )
                .enter_dialogue::<Message, InMemStorage<RegistryState>, RegistryState>()
                .branch(
                    dptree::case![RegistryState::ReceiveEmail]
                        .endpoint(RegistryState::receive_user_email),
                )
                .endpoint(MessageHandler::unauthen_user_handler),
        )
        .branch(Update::filter_inline_query().endpoint(InlineQueryHandler::handler))
        .branch(
            Update::filter_callback_query().branch(
                dptree::filter(CallbackFilter::filter_registry_option)
                    .enter_dialogue::<CallbackQuery, InMemStorage<RegistryState>, RegistryState>()
                    .branch(
                        dptree::case![RegistryState::Start].endpoint(RegistryState::start_handler),
                    )
                    .endpoint(CallbackHandler::reject_registry),
            ),
        )
}
