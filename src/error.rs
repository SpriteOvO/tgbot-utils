#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("No sender chat")]
    NoSenderChat,

    #[error("Could not found media group from database with id '{0}'")]
    MediaGroupNotFound(String),

    #[error("{0}")]
    Internal(#[from] InternalError),
}

#[derive(thiserror::Error, Debug)]
pub enum InternalError {
    #[error("{0}")]
    Teloxide(#[from] TeloxideError),

    #[error("{0}")]
    SerdeJson(serde_json::Error),

    #[error("{0}")]
    Sqlx(sqlx::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum TeloxideError {
    #[error("{0}")]
    Request(#[from] teloxide::errors::RequestError),

    #[error("{0}")]
    Api(#[from] teloxide::errors::ApiError),

    #[error("{0}")]
    Download(#[from] teloxide::errors::DownloadError),
}

pub use crate::cmd_arg::CmdArgError;

macro_rules! impl_from_for_internal_errors {
    ( $( $variant:ident { $($from:ty),+ $(,)? } ),+ $(,)? ) => {
        $(
            $(
            impl From<$from> for InternalError {
                fn from(value: $from) -> Self {
                    Self::$variant(value.into())
                }
            }
            impl From<$from> for Error {
                fn from(value: $from) -> Self {
                    Self::Internal(InternalError::$variant(value.into()))
                }
            }
            )+
        )+
    };
}

impl_from_for_internal_errors! {
    Teloxide { teloxide::errors::RequestError },
    SerdeJson { serde_json::Error },
    Sqlx { sqlx::Error },
}

pub type Result<T> = std::result::Result<T, Error>;
