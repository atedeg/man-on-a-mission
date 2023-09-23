use teloxide::{types::{Message}, Bot, requests::Requester};

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

pub async fn help(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_dice(msg.chat.id).await?;
    Ok(())
}
