//! History, output, and progress state for one sub-agent conversation.
use codetether_provider_core::{ContentPart, Message, Role};
use std::time::{Duration, Instant};

/// Mutable conversation data independent of native execution services.
/// # Examples
/// ```
/// let state = codetether_swarm_core::agent_loop::Conversation::new("system", "user", 60);
/// assert_eq!(state.messages.len(), 2);
/// assert_eq!(state.steps, 0);
/// assert!(state.output.is_empty());
/// ```
pub struct Conversation {
    /// Ordered system, user, assistant, and tool messages.
    pub messages: Vec<Message>,
    /// Completed provider-turn count.
    pub steps: usize,
    /// Number of tool calls started.
    pub tool_calls: usize,
    /// Accumulated assistant output text.
    pub output: String,
    /// Absolute deadline shared by provider and tool work.
    pub deadline: Instant,
    /// Configured model temperature, supplied by native provider policy.
    pub temperature: Option<f32>,
    /// This delegate's position in the shared steering-input log.
    pub steering_cursor: usize,
}

impl Conversation {
    /// Initialize history, counters, output, and the execution deadline.
    /// # Arguments
    /// * `system` — Initial system instructions.
    /// * `user` — Initial task prompt.
    /// * `timeout_secs` — Allowed duration from initialization.
    /// # Returns
    /// State with two messages, zero counters, and no temperature override.
    /// # Panics
    /// Panics if the deadline cannot be represented by [`Instant`].
    /// # Examples
    /// See the construction example on [`Conversation`].
    pub fn new(system: &str, user: &str, timeout_secs: u64) -> Self {
        let text = |role, text: &str| Message {
            role,
            content: vec![ContentPart::Text {
                text: text.to_string(),
            }],
        };
        Self {
            messages: vec![text(Role::System, system), text(Role::User, user)],
            steps: 0,
            tool_calls: 0,
            output: String::new(),
            deadline: Instant::now() + Duration::from_secs(timeout_secs),
            temperature: None,
            steering_cursor: 0,
        }
    }
}
