//! Optional control and steering consumption for delegated agent work.
use crate::ControlHandle;
use codetether_provider_core::Message;

/// Wait for an attached control handle to allow work to proceed.
/// # Arguments
/// * `control` — Parent-session control, when this run has one.
/// # Returns
/// Unit immediately for standalone work, or after the control becomes ready.
/// # Errors
/// Propagates cancellation or readiness errors from the handle.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_session_control::delegated::ready(None).await?;
/// # Ok(()) }
/// ```
pub async fn ready(control: Option<&ControlHandle>) -> anyhow::Result<()> {
    if let Some(control) = control {
        control.ready().await?;
    }
    Ok(())
}

/// Append newly published human input using this delegate's own cursor.
/// # Arguments
/// * `control` — Optional parent control containing accepted input.
/// * `cursor` — This reader's offset, independent of other delegates.
/// * `messages` — Conversation history receiving new input in order.
/// # Returns
/// Whether any messages were appended. Missing control leaves all state unchanged.
/// # Panics
/// Panics if the control's input-log mutex is poisoned.
/// # Examples
/// ```
/// let mut cursor = 0;
/// let mut messages = Vec::new();
/// assert!(!codetether_session_control::delegated::apply_input(None, &mut cursor, &mut messages));
/// assert_eq!(cursor, 0);
/// assert!(messages.is_empty());
/// ```
pub fn apply_input(
    control: Option<&ControlHandle>,
    cursor: &mut usize,
    messages: &mut Vec<Message>,
) -> bool {
    let Some(control) = control else {
        return false;
    };
    let pending = control.inputs_since(cursor);
    let received = !pending.is_empty();
    messages.extend(pending);
    received
}
