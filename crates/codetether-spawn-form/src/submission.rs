//! Form submission decisions independent of the native application session.
use crate::SpawnFormState;
use std::future::Future;
/// Native operations needed to submit and close the spawn form.
/// # Implementors
/// Preserve the existing session/workspace binding in the spawn callback.
pub trait Host {
    /// Display invalid form feedback.
    fn status(&mut self, text: String);
    /// Restore a form whose required name is missing.
    fn restore(&mut self, form: SpawnFormState);
    /// Submit formatted command arguments.
    fn spawn(&mut self, args: &str) -> impl Future<Output = ()>;
    /// Whether the form is still open after command dispatch.
    fn is_open(&self) -> bool;
    /// Close the current form.
    fn close(&mut self);
}
/// Submit form arguments through the existing spawn command, then close it.
/// # Arguments
/// Receives session-bound operations and the completed form state.
/// # Returns
/// Restores invalid input or completes after spawn command dispatch.
/// # Examples
/// ```no_run
/// async fn submit<H: codetether_spawn_form::submission::Host>(host: &mut H) {
///     codetether_spawn_form::submission::submit(host, Default::default()).await;
/// }
/// ```
pub async fn submit<H: Host>(app: &mut H, form: SpawnFormState) {
    let Some(args) = form.to_args() else {
        app.status("Name is required".to_string());
        app.restore(form);
        return;
    };
    app.spawn(&args).await;
    if app.is_open() {
        app.close();
    }
}
