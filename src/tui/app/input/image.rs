//! Frontend type selection for pending-image attachment and feedback.
pub(crate) use codetether_image::{attachment_from_file as attach_image_file, image_summary};
codetether_image::declare_frontend_image_attachment!(
    crate::tui::app::state::App,
    crate::tui::chat::message::ChatMessage,
    crate::tui::chat::message::MessageType,
);
