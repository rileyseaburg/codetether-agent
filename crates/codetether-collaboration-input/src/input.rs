//! Validation and normalization of collaboration input items.

use super::item::InputItem;
use crate::MessageImage;
use anyhow::{Result, bail};
use std::path::Path;

#[cfg(test)]
#[path = "tests/images.rs"]
mod images_tests;
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

/// Normalized text and durable image attachments ready for submission.
///
/// # Examples
/// ```
/// let input=codetether_collaboration_input::Prepared {message:"Hello".into(),images:vec![]};
/// assert!(input.images.is_empty());
/// ```
pub struct Prepared {
    /// Text prompt with image placeholders and rendered references.
    pub message: String,
    /// Attachments retained in input order.
    pub images: Vec<MessageImage>,
}

#[cfg(test)]
pub(super) async fn prepare(
    message: Option<String>,
    items: Option<Vec<InputItem>>,
) -> Result<Prepared> {
    prepare_in_workspace(message, items, None).await
}

/// Validate and normalize either plain text or structured input items.
///
/// # Arguments
/// * `message` — Plain text, mutually exclusive with items.
/// * `items` — Structured input, mutually exclusive with message.
/// * `workspace` — Base directory for relative local-image paths.
/// # Returns
/// Text plus durable attachments in their original order.
/// # Errors
/// Returns invalid-input, unsupported-audio, invalid-image or file-read errors.
/// # Examples
/// ```no_run
/// # async fn example()->anyhow::Result<()> {
/// let input=codetether_collaboration_input::prepare_in_workspace(Some("Hello".into()),None,None).await?;
/// assert_eq!(input.message,"Hello");
/// # Ok(()) }
/// ```
pub async fn prepare_in_workspace(
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
