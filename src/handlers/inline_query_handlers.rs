use teloxide::{
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText,
    },
};

use crate::types::TeloxideError;

pub struct InlineQueryHandler;

impl InlineQueryHandler {
    pub async fn handler(bot: Bot, query: InlineQuery) -> Result<(), TeloxideError> {
        let options = ["Yes", "No"];

        bot.answer_inline_query(
            query.id,
            vec![InlineQueryResultArticle::new(
                "create-account-inline-query",
                "Create account decision",
                InputMessageContent::Text(InputMessageContentText::new("Create account decision")),
            )
            .reply_markup(InlineKeyboardMarkup::new(vec![options.iter().map(
                |option| InlineKeyboardButton::callback(option.to_owned(), option.to_owned()),
            )]))
            .into()],
        )
        .await?;

        Ok(())
    }
}
