use teloxide::{
    prelude::*,
    types::{MessageCommon, MessageKind},
};

use crate::{
    models::User,
    types::{RegistryDialogue, TeloxideError},
};

#[derive(Clone, Default)]
pub enum RegistryState {
    #[default]
    Start,
    ReceiveEmail,
}

impl RegistryState {
    pub async fn start_handler(
        bot: Bot,
        query: CallbackQuery,
        dialogue: RegistryDialogue,
    ) -> Result<(), TeloxideError> {
        bot.answer_callback_query(query.id).await?;

        let text = "Great! We need your email for future features. What is your email?";
        if let Some(Message { id, chat, .. }) = query.message {
            bot.edit_message_text(chat.id, id, text).await?;
        } else if let Some(id) = query.inline_message_id {
            bot.edit_message_text_inline(id, text).await?;
        }

        dialogue.update(RegistryState::ReceiveEmail).await?;
        Ok(())
    }

    pub async fn receive_user_email(
        bot: Bot,
        msg: Message,
        dialogue: RegistryDialogue,
    ) -> Result<(), TeloxideError> {
        match (&msg.kind, msg.text()) {
            (
                MessageKind::Common(MessageCommon {
                    from: Some(user), ..
                }),
                Some(text),
            ) => {
                let uid = user.id.0;

                dialogue.exit().await?;

                match User::create_user(uid.to_string(), text.to_string()) {
                    Ok(_) => {
                        let reply =
                            "Great! Now you can access our system. Feel free to ask anything 😉";
                        bot.send_message(msg.chat.id, reply).await?;
                    }
                    Err(_) => {
                        let reply = "Sorry, an unknown error occurred while trying to create an account for you :(";
                        bot.send_message(msg.chat.id, reply).await?;
                    }
                }
            }
            _ => {
                bot.send_message(msg.chat.id, "Send me plain text only.")
                    .await?;
            }
        }
        Ok(())
    }
}
