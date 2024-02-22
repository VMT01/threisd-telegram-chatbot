use teloxide::{
    dispatching::{UpdateFilterExt, UpdateHandler},
    prelude::*,
    types::{
        InlineKeyboardButton, InlineKeyboardMarkup, InlineQueryResultArticle, InputMessageContent,
        InputMessageContentText, MessageKind, Update,
    },
    RequestError,
};

use crate::models::User;

pub fn build_handler() -> UpdateHandler<RequestError> {
    dptree::entry()
        .branch(Update::filter_message().endpoint(login))
        .branch(Update::filter_inline_query().endpoint(inline_query_handler))
        .branch(Update::filter_callback_query().endpoint(callback_handler))
}

fn make_choice_keyboard() -> InlineKeyboardMarkup {
    let keyboard: Vec<InlineKeyboardButton> = ["Yes", "No"]
        .iter()
        .map(|&item| InlineKeyboardButton::callback(item.to_owned(), item.to_owned()))
        .collect();
    InlineKeyboardMarkup::new(vec![keyboard])
}

/// Kiểm tra người dùng có tồn tại trong DB hay không
async fn login(bot: Bot, msg: Message) -> Result<(), RequestError> {
    let chat_id = msg.chat.id;
    let mut user_id: u64 = 0;

    if let MessageKind::Common(message) = msg.kind {
        UserId(user_id) = message.from.unwrap().id;
    }

    if let Some(user) = User::find_user(user_id.to_string()) {
        println!("{:?}", user);
        bot.send_message(msg.chat.id, "Hello World").await?;
    } else {
        let message = "Look like you don't appear in our system. To use our utilities, you must create and account. Do you agree?";
        bot.send_message(chat_id, message)
            .reply_markup(make_choice_keyboard())
            .await?;
    }

    Ok(())
}

async fn inline_query_handler(bot: Bot, q: InlineQuery) -> Result<(), RequestError> {
    let choice = InlineQueryResultArticle::new(
        "0",
        "Choice",
        InputMessageContent::Text(InputMessageContentText::new("You chose:")),
    )
    .reply_markup(make_choice_keyboard());
    bot.answer_inline_query(q.id, vec![choice.into()]).await?;

    Ok(())
}

async fn callback_handler(bot: Bot, q: CallbackQuery) -> Result<(), RequestError> {
    let chat_id = q.clone().message.unwrap().chat.id;

    if let Some(choice) = q.data {
        bot.answer_callback_query(q.id).await?;

        let UserId(user_id) = q.from.id;
        let res = User::create_user(user_id.to_string());

        if choice.eq("Yes") {
            let text = "Great! Let me prepare a bit.";
            if let Some(Message { id, chat, .. }) = q.message {
                bot.edit_message_text(chat.id, id, text).await?;
            } else if let Some(id) = q.inline_message_id {
                bot.edit_message_text_inline(id, text).await?;
            }

            if res.is_some() {
                bot.send_message(chat_id, "Congrat! You have an account!")
                    .await?;
            } else {
                bot.send_message(chat_id, "Sorry, there's an unexpected error occurred while trying to get you an account.").await?;
            }
        } else {
            let text = "Too bad, you cannot use our service without any account. Anyway, it's your choice. Have fun :)";
            if let Some(Message { id, chat, .. }) = q.message {
                bot.edit_message_text(chat.id, id, text).await?;
            } else if let Some(id) = q.inline_message_id {
                bot.edit_message_text_inline(id, text).await?;
            }
        }
    }
    Ok(())
}
