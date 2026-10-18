use super::Host;
pub(super) async fn resume<H: Host>(host: &mut H, prompt: String) {
    host.status("Continuing — resubmitting prompt…".to_string());
    host.publish_user(&prompt);
    host.dispatch(&prompt).await;
}
