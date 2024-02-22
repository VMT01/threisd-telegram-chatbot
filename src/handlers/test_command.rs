use teloxide::{
    dispatching::{HandlerExt, UpdateHandler},
    prelude::*,
    utils::command::BotCommands,
    RequestError,
};

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum TestCommand {
    #[command(description = "Display this text.")]
    Help,

    #[command(description = "Handle a username.")]
    Username(String),

    #[command(description = "Handle a username and an age.", parse_with = "split")]
    UsernameAndAge { username: String, age: u8 },
}

async fn handler(bot: Bot, msg: Message, cmd: TestCommand) -> Result<(), RequestError> {
    match cmd {
        TestCommand::Help => {
            bot.send_message(msg.chat.id, TestCommand::descriptions().to_string())
                .await?
        }
        TestCommand::Username(username) => {
            bot.send_message(msg.chat.id, format!("Your username is @{username}."))
                .await?
        }
        TestCommand::UsernameAndAge { username, age } => {
            bot.send_message(
                msg.chat.id,
                format!("Your username is @{username} and age is {age}."),
            )
            .await?
        }
    };
    Ok(())
}

pub fn build_handler() -> UpdateHandler<RequestError> {
    Update::filter_message().branch(
        dptree::entry()
            .filter_command::<TestCommand>()
            .endpoint(handler),
    )
}
