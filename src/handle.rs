use teloxide::{prelude::*, types::Me};

use crate::{button::*, text::*};

pub enum RequestKind<C> {
    NewMessage(Message),
    EditedMessage(Message),
    Command(Message, C),
    CallbackQuery(CallbackQuery),
}

pub struct Request<S, C> {
    state: S,
    bot: Bot,
    me: Me,
    kind: RequestKind<C>,
}

impl<S, C> Request<S, C> {
    pub fn new_message(state: S, bot: Bot, me: Me, msg: Message) -> Self {
        Self {
            state,
            bot,
            me,
            kind: RequestKind::NewMessage(msg),
        }
    }

    pub fn edited_message(state: S, bot: Bot, me: Me, msg: Message) -> Self {
        Self {
            state,
            bot,
            me,
            kind: RequestKind::EditedMessage(msg),
        }
    }

    pub fn callback_query(state: S, bot: Bot, me: Me, callback_query: CallbackQuery) -> Self {
        Self {
            state,
            bot,
            me,
            kind: RequestKind::CallbackQuery(callback_query),
        }
    }

    pub fn new_command(state: S, bot: Bot, me: Me, msg: Message, cmd: C) -> Self {
        Self {
            state,
            bot,
            me,
            kind: RequestKind::Command(msg, cmd),
        }
    }

    pub fn state(&self) -> &S {
        &self.state
    }

    pub fn bot(&self) -> &Bot {
        &self.bot
    }

    pub fn me(&self) -> &Me {
        &self.me
    }

    // pub fn msg(&self) -> Option<&Message> {
    //     match self.kind {
    //         RequestKind::NewMessage(ref msg) | RequestKind::EditedMessage(ref
    // msg) => Some(msg),         _ => None,
    //     }
    // }

    pub fn kind(&self) -> &RequestKind<C> {
        &self.kind
    }
}

#[derive(Debug)]
pub enum ResponseKind<'a> {
    Nothing,
    ReplyTo(MessageText<'a>, Option<MessageButtons>),
    NewMsg(MessageText<'a>, Option<MessageButtons>),
    Popup(String),
}

pub struct Response<'a> {
    pub kind: ResponseKind<'a>,
}

impl<'a> Response<'a> {
    pub fn nothing() -> Self {
        Self {
            kind: ResponseKind::Nothing,
        }
    }

    pub fn reply_to(text: impl Into<MessageText<'a>>) -> Self {
        Self {
            kind: ResponseKind::ReplyTo(text.into(), None),
        }
    }

    pub fn reply_to_with_button(
        text: impl Into<MessageText<'a>>,
        buttons: impl Into<MessageButtons>,
    ) -> Self {
        Self {
            kind: ResponseKind::ReplyTo(text.into(), Some(buttons.into())),
        }
    }

    pub fn new_msg(text: impl Into<MessageText<'a>>) -> Self {
        Self {
            kind: ResponseKind::NewMsg(text.into(), None),
        }
    }

    pub fn new_msg_with_button(
        text: impl Into<MessageText<'a>>,
        buttons: impl Into<MessageButtons>,
    ) -> Self {
        Self {
            kind: ResponseKind::NewMsg(text.into(), Some(buttons.into())),
        }
    }

    pub fn popup(text: impl Into<String>) -> Self {
        Self {
            kind: ResponseKind::Popup(text.into()),
        }
    }
}
