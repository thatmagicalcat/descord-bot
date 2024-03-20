mod reaction_add;
mod ready;

lazy_static::lazy_static! {
    pub(crate) static ref BOT_ID: tokio::sync::Mutex<String> = tokio::sync::Mutex::new(String::new());
}

pub use reaction_add::*;
pub use ready::*;
