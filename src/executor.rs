use teloxide::{
    payloads::SendMessage,
    prelude::*,
    requests::JsonRequest,
    types::{ChatId, ReplyMarkup},
};

use crate::{button::*, text::*};

pub struct MessageExecutor<'a> {
    bot: &'a Bot,
    text: MessageText<'a>,
    buttons: Option<MessageButtons>,
}

impl<'a> MessageExecutor<'a> {
    pub fn new(bot: &'a Bot, text: MessageText<'a>, buttons: Option<MessageButtons>) -> Self {
        Self { bot, text, buttons }
    }

    pub fn send_message(self, chat_id: ChatId) -> JsonRequest<SendMessage> {
        let entities: Vec<_> = self.text.entities().into();

        let mut builder = self
            .bot
            .send_message(chat_id, self.text.text())
            .entities(entities)
            .disable_web_page_preview(self.text.disable_preview());
        if let Some(buttons) = self.buttons {
            builder = builder.reply_markup::<ReplyMarkup>(buttons.into())
        }
        builder
    }
}
