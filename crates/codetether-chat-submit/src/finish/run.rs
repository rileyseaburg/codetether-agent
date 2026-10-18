use super::Actions;
/// Finalize a prompt after chat routing selects normal submission.
/// # Arguments
/// * `host` — Native attachment, goal, chat and dispatch operations.
/// * `prompt` — Compact user input before paste-placeholder expansion.
/// # Returns
/// Completes after empty-input rejection or provider dispatch returns.
/// # Examples
/// ```ignore
/// finish(&mut native_chat, "Please review this paste").await;
/// ```
pub async fn finish<H: Actions>(host: &mut H, prompt: &str) {
    let pending_images = host.take_images();
    let pending_text_pastes = host.take_text();
    if prompt.is_empty() && pending_images.is_empty() {
        return;
    }
    // Chat history shows the compact placeholder; the agent receives the
    // expanded full paste content wrapped in delimiters.
    let agent_prompt = host.expand(prompt, &pending_text_pastes);
    host.adopt(&agent_prompt).await;
    host.publish(prompt, &pending_images);
    host.dispatch(&agent_prompt, pending_images).await;
}
