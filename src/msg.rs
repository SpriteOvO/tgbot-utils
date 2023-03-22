use teloxide::{prelude::*, types::Message};

use crate::error::*;

// expensive
pub async fn is_from_linked_channel(bot: &Bot, msg: &Message) -> Result<bool> {
    let sender_chat = msg.sender_chat().ok_or(Error::NoSenderChat)?;
    let channel_id = bot.get_chat(msg.chat.id).await?.linked_chat_id();

    Ok(channel_id == Some(sender_chat.id.0))
}
