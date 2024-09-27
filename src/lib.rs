use teloxide::types::{ChatId, Recipient};

pub mod configuration;
pub mod handlers;

pub struct WrappedI64(pub i64);

impl From<WrappedI64> for Recipient {
    fn from(value: WrappedI64) -> Self {
        Self::Id(ChatId(value.0))
    }
}
