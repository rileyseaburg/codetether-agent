use super::Host;
/// Inject pending input into an active run, or retain it for a future turn.
/// # Arguments
/// Host binds the runtime and UI; prompt is compact raw input.
/// # Returns
/// Updates UI state only according to whether steering was accepted.
/// # Examples
/// ```ignore
/// submit(&mut native_ui, "Use the cloned repositories");
/// ```
pub fn submit<H: Host>(host: &mut H, prompt: &str) {
    let images = host.images();
    if prompt.is_empty() && images.is_empty() {
        return;
    }
    let expanded = host.expand(prompt);
    if !host.steer(expanded, images.clone()) {
        queue_fallback(host, prompt);
        return;
    }
    host.clear_attachments();
    crate::publication::push_user_messages(host.messages(), prompt, &images);
    host.clear_input();
    host.status("Steered current turn".to_string());
    host.scroll();
}

fn queue_fallback<H: Host>(host: &mut H, prompt: &str) {
    if prompt.is_empty() {
        host.status("Image ready for the next turn".to_string());
    } else {
        host.queue(prompt.to_string());
    }
}
