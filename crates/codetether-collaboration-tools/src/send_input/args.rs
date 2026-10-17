//! Deserialization contract for `send_input`.

use crate::RuntimeContext;
use codetether_collaboration_input::InputItem;
use serde::Deserialize;

#[derive(Deserialize)]
/// Structured submission arguments with trusted parent runtime context.
///
/// # Examples
/// ```
/// let args:codetether_collaboration_tools::SendInputArgs=serde_json::from_value(
///     serde_json::json!({"target":"child","message":"Hello"})).unwrap();
/// assert!(!args.interrupt);
/// ```
pub struct SendInputArgs {
    /// Child identity resolved by the existing agent runtime.
    pub target: String,
    #[serde(default)]
    /// Plain-text input, mutually exclusive with items.
    pub message: Option<String>,
    #[serde(default)]
    /// Structured text and image input, mutually exclusive with message.
    pub items: Option<Vec<InputItem>>,
    #[serde(default)]
    /// Whether to interrupt the child before submitting input.
    pub interrupt: bool,
    #[serde(flatten)]
    /// Trusted parent session, model and workspace fields.
    pub context: RuntimeContext,
}
