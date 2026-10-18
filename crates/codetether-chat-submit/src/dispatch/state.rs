/// Mutable dispatch status borrowed from the native UI.
/// # Examples
/// ```
/// let mut restarts = 2;
/// let mut prompt = None;
/// let mut status = String::new();
/// let state = codetether_chat_submit::dispatch::State {
///     watchdog_restarts: &mut restarts, inflight_prompt: &mut prompt, status: &mut status,
/// };
/// assert_eq!(*state.watchdog_restarts, 2);
/// ```
pub struct State<'a> {
    /// Number of automatic watchdog restarts since the last user prompt.
    pub watchdog_restarts: &'a mut u32,
    /// Prompt retained for in-flight recovery.
    pub inflight_prompt: &'a mut Option<String>,
    /// Status text displayed by the UI.
    pub status: &'a mut String,
}
