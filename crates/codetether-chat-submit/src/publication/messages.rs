use codetether_chat_model::{ChatMessage, MessageType};
use codetether_image::ImageAttachment;
/// Append compact user text followed by image summaries in attachment order.
/// # Arguments
/// Messages is the displayed chat log; prompt is unexpanded user input;
/// images retain the native attachment data and summary policy.
/// # Returns
/// Appends messages with current timestamps and no completion usage.
/// # Examples
/// ```
/// let mut messages = Vec::new();
/// codetether_chat_submit::publication::push_user_messages(&mut messages, "hello", &[]);
/// assert_eq!(messages[0].content, "hello");
/// ```
pub fn push_user_messages(
    messages: &mut Vec<ChatMessage>,
    prompt: &str,
    images: &[ImageAttachment],
) {
    if !prompt.is_empty() {
        messages.push(ChatMessage::new(MessageType::User, prompt.to_string()));
    }
    for image in images {
        let summary = codetether_image::image_summary(image);
        messages.push(ChatMessage::new(
            MessageType::Image {
                url: image.data_url.clone(),
            },
            summary,
        ));
    }
}
