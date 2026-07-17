//! One user input submitted while a prompt run is active.

use crate::provider::{ContentPart, Message, Role};
use crate::session::ImageAttachment;

/// Text and images to inject before the active run's next model step.
#[derive(Debug, Clone)]
pub(crate) struct SteeringInput {
    text: String,
    images: Vec<ImageAttachment>,
}

impl SteeringInput {
    /// Create a steering input from expanded text and image attachments.
    pub(crate) fn new(text: String, images: Vec<ImageAttachment>) -> Self {
        Self { text, images }
    }

    /// Convert the input into a trusted user-role transcript message.
    pub(super) fn into_message(self) -> (Message, String) {
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
