use std::future::Future;
/// Select steering or normal submission after higher-priority handlers.
/// # Implementors
/// Native state supplies processing status and the existing dispatch paths.
pub trait Submission {
    /// Whether an existing turn is being processed.
    fn processing(&self) -> bool;
    /// Steer the active turn with this prompt.
    fn steer(&mut self, prompt: &str);
    /// Finish ordinary submission, including attachments and provider dispatch.
    fn finish(&mut self, prompt: &str) -> impl Future<Output = ()>;
}
