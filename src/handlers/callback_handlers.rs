use teloxide::prelude::*;

use crate::types::TeloxideError;

pub struct CallbackHandler;

impl CallbackHandler {
    pub async fn reject_registry(bot: Bot, query: CallbackQuery) -> Result<(), TeloxideError> {
        bot.answer_callback_query(query.id).await?;

        let text = "Too bad :( I cannot grant you permission to use our system";
        if let Some(Message { id, chat, .. }) = query.message {
            bot.edit_message_text(chat.id, id, text).await?;
        } else if let Some(id) = query.inline_message_id {
            bot.edit_message_text_inline(id, text).await?;
        }
        Ok(())
    }
}

pub struct CallbackFilter;

impl CallbackFilter {
    pub fn filter_registry_option(query: CallbackQuery) -> bool {
        if let Some(data) = query.data {
            return data.eq("Yes");
        }
        false
    }
}
