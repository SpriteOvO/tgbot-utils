pub mod cmd_arg;
pub mod error;
pub mod handle;
pub mod media;
mod msg;
mod prog_msg;
pub mod text;

pub use error::{Error, Result};
pub use msg::*;
pub use prog_msg::*;
use sqlx::SqlitePool;

pub trait DbPoolCallback<'a>: Fn() -> &'a SqlitePool {}

impl<'a, T> DbPoolCallback<'a> for T where T: Fn() -> &'a SqlitePool {}
