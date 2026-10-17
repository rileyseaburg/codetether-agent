//! Conversation messages passed across the host boundary.

use super::{LlmResponse, ToolCall};

/// A message in the conversation.
///
/// # Examples
/// ```
/// use codetether_rlm::{traits::LlmMessage};
/// assert_eq!(LlmMessage::user("hello".into()).role, "user");
/// ```
#[derive(Debug, Clone)]
pub struct LlmMessage {
    pub role: String,
    pub text: String,
    pub tool_calls: Vec<ToolCall>,
    pub tool_call_id: Option<String>,
}

impl LlmMessage {
    /// Create a user message.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{traits::LlmMessage};
    /// assert_eq!(LlmMessage::user("hello".into()).role, "user");
    /// ```
    pub fn user(text: String) -> Self {
        Self {
            role: "user".into(),
            text,
            tool_calls: vec![],
            tool_call_id: None,
        }
    }
    /// Create an assistant message (text only).
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{traits::LlmMessage};
    /// assert_eq!(LlmMessage::assistant("hello".into()).role, "assistant");
    /// ```
    pub fn assistant(text: String) -> Self {
        Self {
            role: "assistant".into(),
            text,
            tool_calls: vec![],
            tool_call_id: None,
        }
    }
    /// Create an assistant message from a response.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{traits::{LlmMessage,LlmResponse}};
    /// let response = LlmResponse { text: "hello".into(), tool_calls: vec![], finish_reason: None, input_tokens: 1, output_tokens: 1 };
    /// assert_eq!(LlmMessage::assistant_from(&response).text, "hello");
    /// ```
    pub fn assistant_from(resp: &LlmResponse) -> Self {
        Self {
            role: "assistant".into(),
            text: resp.text.clone(),
            tool_calls: resp.tool_calls.clone(),
            tool_call_id: None,
        }
    }
    /// Create a tool-result message.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{traits::LlmMessage};
    /// let message = LlmMessage::tool_result("call", "done");
    /// assert_eq!(message.role, "tool");
    /// assert_eq!(message.tool_call_id.as_deref(), Some("call"));
    /// ```
    pub fn tool_result(call_id: &str, content: &str) -> Self {
        Self {
            role: "tool".into(),
            text: content.into(),
            tool_calls: vec![],
            tool_call_id: Some(call_id.into()),
        }
    }
}
