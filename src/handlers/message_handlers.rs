use teloxide::{
    prelude::*,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, MessageCommon, MessageKind},
};

use crate::{models::User, types::TeloxideError};

pub struct MessageHandler;

impl MessageHandler {
    pub async fn authen_user_handler(bot: Bot, msg: Message) -> Result<(), TeloxideError> {
        bot.send_message(msg.chat.id, "Hello World!").await?;
        Ok(())
    }

    pub async fn unauthen_user_handler(bot: Bot, msg: Message) -> Result<(), TeloxideError> {
        let message = "Looks like you don't appear in our system. To use our facilities, you must create an account. Do you agree?";
        let options = ["Yes", "No"];

        bot.send_message(msg.chat.id, message)
            .reply_markup(InlineKeyboardMarkup::new(vec![options.iter().map(
                |option| InlineKeyboardButton::callback(option.to_owned(), option.to_owned()),
            )]))
            .await?;
        Ok(())
    }
}

pub struct MessageFilter;

impl MessageFilter {
    pub fn filter_valid_user(msg: Message) -> bool {
        match msg.kind {
            MessageKind::Common(MessageCommon {
                from: Some(user), ..
            }) => {
                let user_id = user.id.0;
                User::find_one_by_user_id(user_id.to_string()).is_some()
            }
            _ => false,
        }
    }
}
