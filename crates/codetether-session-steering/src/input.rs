//! One user input submitted while a prompt run is active.

use codetether_image::ImageAttachment;
use codetether_provider_core::{ContentPart, Message, Role};
use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_ACTIVITY_ID: AtomicU64 = AtomicU64::new(1);

/// Text and images to inject before the active run's next model step.
/// # Examples
/// ```rust
/// let input = codetether_session_steering::SteeringInput::new("Review this".into(), vec![]);
/// # let _ = input;
/// ```
#[derive(Debug, Clone)]
pub struct SteeringInput {
    activity_id: u64,
    text: String,
    images: Vec<ImageAttachment>,
}

impl SteeringInput {
    /// Create a steering input from expanded text and image attachments.
    /// # Arguments
    /// `text` is expanded user text; `images` are ordered image attachments.
    /// # Returns
    /// An input with a fresh process-local activity identity.
    /// # Examples
    /// ```rust
    /// let input = codetether_session_steering::SteeringInput::new("Continue".into(), vec![]);
    /// # let _ = input;
    /// ```
    pub fn new(text: String, images: Vec<ImageAttachment>) -> Self {
        Self {
            activity_id: NEXT_ACTIVITY_ID.fetch_add(1, Ordering::Relaxed),
            text,
            images,
        }
    }

    pub(crate) fn activity_id(&self) -> u64 {
        self.activity_id
    }

    /// Convert the input into a trusted user-role transcript message.
    pub(crate) fn into_message(self) -> (Message, String) {
        let mut content = vec![ContentPart::Text {
            text: self.text.clone(),
        }];
        content.extend(self.images.into_iter().map(|image| ContentPart::Image {
            url: image.data_url,
            mime_type: image.mime_type,
        }));
        (
            Message {
                role: Role::User,
                content,
            },
            self.text,
        )
    }
}
