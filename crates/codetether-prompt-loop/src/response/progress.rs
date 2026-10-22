//! Guidance from durable observed results, independent of file-writing activity.
use codetether_provider_core::Message;
/// Session evidence and transcript access used after completed tool batches.
/// # Implementors
/// Native runners borrow the ledger stored in session metadata.
pub trait Host {
    /// Borrow durable observations shared across goal continuations and restarts.
    fn evidence(&mut self) -> &mut codetether_repeat_guard::ProgressLedger;
    /// Append corrective guidance through the existing session path.
    fn add_message(&mut self, message: Message);
}
/// Review task continuity periodically, and nudge for exact repeated evidence.
/// # Arguments
/// `host` supplies the persisted ledger; `step` identifies the current iteration.
/// # Returns
/// Unit; this guidance does not stop or declare completion of the task.
/// # Examples
/// ```no_run
/// # fn example(host: &mut impl codetether_prompt_loop::response::progress::Host) {
/// codetether_prompt_loop::response::progress::track(host, 1);
/// # }
/// ```
pub fn track(host: &mut impl Host, step: usize) {
    let interval = codetether_prompt_policy::loop_constants::MAX_STEPS_WITHOUT_PROGRESS;
    let evidence = host.evidence();
    let message = evidence
        .take_review(interval)
        .or_else(|| evidence.take_nudge(interval).map(str::to_owned));
    if let Some(message) = message {
        tracing::info!(step, "Reviewing task continuity after tool results");
        host.add_message(super::corrective_message(&message));
    }
}
