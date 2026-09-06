//! Validation and normalization of collaboration input items.

use super::item::InputItem;
use crate::tool::agent::collaboration_runtime::message_input::MessageImage;
use anyhow::{Result, bail};
use std::path::Path;

#[cfg(test)]
#[path = "tests/images.rs"]
mod images_tests;
#[cfg(test)]
#[path = "tests/local_images.rs"]
mod local_images_tests;
#[cfg(test)]
#[path = "tests/remote_images.rs"]
mod remote_images_tests;
#[cfg(test)]
#[path = "tests/text.rs"]
mod text_tests;

#[path = "input/image.rs"]
mod image;
#[path = "input/render.rs"]
mod render;

pub(super) struct Prepared {
    pub(super) message: String,
    pub(super) images: Vec<MessageImage>,
}

#[cfg(test)]
pub(super) async fn prepare(
    message: Option<String>,
    items: Option<Vec<InputItem>>,
) -> Result<Prepared> {
    prepare_in_workspace(message, items, None).await
}

pub(super) async fn prepare_in_workspace(
    message: Option<String>,
    items: Option<Vec<InputItem>>,
    workspace: Option<&Path>,
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
        (None, Some(items)) => render::items(items, workspace).await,
    }
}
