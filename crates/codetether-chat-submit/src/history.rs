//! Capture unexpanded raw chat input in native prompt history.
/// Input access and native history storage used by [capture].
/// # Implementors
/// UI adapters keep their existing history deduplication and navigation policy.
pub trait History {
    /// Borrow current raw input before mention or alias expansion.
    fn input(&self) -> &str;
    /// Record a nonempty trimmed prompt.
    fn push_history(&mut self, prompt: String);
}
/// Trim raw input, record nonempty history and return the captured prompt.
/// # Arguments
/// Host supplies current input and the existing history insertion operation.
/// # Returns
/// The trimmed, otherwise unmodified prompt string.
/// # Examples
/// ```ignore
/// let prompt = capture(&mut ui);
/// ```
pub fn capture<H: History>(host: &mut H) -> String {
    let prompt = host.input().trim().to_string();
    if !prompt.is_empty() {
        host.push_history(prompt.clone());
    }
    prompt
}
