use super::Host;
use codetether_image::ImageAttachment;
use codetether_provider_core::{ContentPart, Message, Role};
/// Append human content, publish its text and generate a missing title.
/// # Arguments
/// `host` supplies session services; `message` is original text; `images`
/// contains attachments in user-provided order.
/// # Returns
/// Success after history insertion, publication and optional title generation.
/// # Errors
/// Propagates title-generation errors after earlier effects have completed.
/// # Examples
/// ```no_run
/// # async fn input(host: &mut impl codetether_prompt_loop::input::Host) -> anyhow::Result<()> {
/// codetether_prompt_loop::input::accept(host, "Hello", Vec::new()).await?;
/// # Ok(()) }
/// ```
pub async fn accept(
    host: &mut impl Host,
    message: &str,
    images: Vec<ImageAttachment>,
) -> anyhow::Result<()> {
    let mut content = vec![ContentPart::Text {
        text: message.to_string(),
    }];
    content.extend(images.iter().map(|image| ContentPart::Image {
        url: image.data_url.clone(),
        mime_type: image.mime_type.clone(),
    }));
    if !images.is_empty() {
        tracing::info!(image_count = images.len(), "Adding image attachments");
    }
    host.append_human(Message {
        role: Role::User,
        content,
    });
    let turn_id = host.turn_id().to_owned();
    host.publish(message, turn_id);
    if host.needs_title() {
        host.generate_title().await?;
    }
    Ok(())
}
