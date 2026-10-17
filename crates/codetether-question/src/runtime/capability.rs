//! Task-local opt-in for exactly one foreground question interaction.
use std::future::Future;
tokio::task_local! { pub(super) static SESSION: String; }
/// Opt a single foreground future into question interaction.
///
/// # Arguments
/// * `session` — Exact session ID rendered by the attached frontend.
/// * `future` — Turn to run. Dropping it cancels any pending question.
/// # Returns
/// The future's output, unchanged.
/// # Examples
/// ```no_run
/// # async fn demo() {
/// codetether_question::runtime::scope("session-1".into(), async {
///     assert!(codetether_question::runtime::available());
/// }).await;
/// # }
/// ```
pub async fn scope<T>(session: String, future: impl Future<Output = T>) -> T {
    SESSION.scope(session, future).await
}
/// Whether the current task has an attached frontend; false in headless tasks.
///
/// # Arguments
/// None.
/// # Returns
/// True only inside [`scope`].
/// # Examples
/// ```
/// assert!(!codetether_question::runtime::available());
/// ```
pub fn available() -> bool {
    SESSION.try_with(|_| ()).is_ok()
}
