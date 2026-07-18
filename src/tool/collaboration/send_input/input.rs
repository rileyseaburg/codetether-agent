//! Validation and normalization of collaboration input items.

use super::item::InputItem;
use crate::tool::agent::collaboration_runtime::message_input::MessageImage;
use anyhow::{Result, bail};

#[path = "input/image.rs"]
mod image;
#[path = "input/render.rs"]
mod render;

pub(super) struct Prepared {
    pub(super) message: String,
    pub(super) images: Vec<MessageImage>,
}

pub(super) async fn prepare(
    message: Option<String>,
    items: Option<Vec<InputItem>>,
) -> Result<Prepared> {
    match (message, items) {
        (Some(_), Some(_)) => bail!("Provide either message or items, but not both"),
        (None, None) => bail!("Provide one of: message or items"),
        (Some(message), None) if message.trim().is_empty() => {
            bail!("Empty message can't be sent to an agent")
        }
        (Some(message), None) => Ok(Prepared {
            message,
            images: Vec::new(),
        }),
        (None, Some(items)) if items.is_empty() => bail!("Items can't be empty"),
        (None, Some(items)) => render::items(items).await,
    }
}
