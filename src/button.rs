use std::fmt::Debug;

use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup, ReplyMarkup};

pub trait MessageButtonAction: Send + Sync + Debug {
    fn ser(&self) -> String;
    fn deser(input: &str) -> Option<Self>
    where
        Self: Sized;
}
#[derive(Debug)]
pub struct MessageButton {
    text: String,
    action: Box<dyn MessageButtonAction>,
}

impl MessageButton {
    pub fn new(text: impl Into<String>, action: Box<dyn MessageButtonAction>) -> Self {
        Self {
            text: text.into(),
            action,
        }
    }
}

impl From<MessageButton> for InlineKeyboardButton {
    fn from(value: MessageButton) -> Self {
        InlineKeyboardButton::callback(value.text, value.action.ser())
    }
}

#[derive(Debug)]
pub struct MessageButtons(Vec<Vec<MessageButton>>);

impl MessageButtons {
    pub fn new(
        buttons: impl IntoIterator<Item = impl IntoIterator<Item = impl Into<MessageButton>>>,
    ) -> Self {
        Self(
            buttons
                .into_iter()
                .map(|b| b.into_iter().map(|b| b.into()).collect())
                .collect(),
        )
    }

    fn into_inline_keyboard_buttons(
        self,
    ) -> impl IntoIterator<Item = impl IntoIterator<Item = InlineKeyboardButton>> {
        self.0.into_iter().map(|b| b.into_iter().map(|b| b.into()))
    }
}

impl From<MessageButtons> for InlineKeyboardMarkup {
    fn from(value: MessageButtons) -> Self {
        InlineKeyboardMarkup::new(value.into_inline_keyboard_buttons())
    }
}

impl From<MessageButtons> for ReplyMarkup {
    fn from(value: MessageButtons) -> Self {
        ReplyMarkup::inline_kb(value.into_inline_keyboard_buttons())
    }
}
