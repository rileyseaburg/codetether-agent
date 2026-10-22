//! Observe actual transcript appends, never proposed calls as completed work.
use super::{ProgressLedger, ledger::Pending, ledger_signature::signature};
use codetether_provider_core::{ContentPart, Message, Role};
impl ProgressLedger {
    /// Record calls and their matching results at an absolute transcript position.
    /// # Arguments
    /// `message` is the original appended message; `source_turn` is its absolute index.
    /// # Returns
    /// Unit; pending and observed entries are bounded to 128 each.
    /// # Examples
    /// ```
    /// use codetether_provider_core::{Message, Role};
    /// let mut ledger = codetether_repeat_guard::ProgressLedger::default();
    /// ledger.observe_message(&Message { role: Role::Assistant, content: vec![] }, 0);
    /// assert_eq!(ledger.observations, 0);
    /// ```
    pub fn observe_message(&mut self, message: &Message, source_turn: usize) {
        for part in &message.content {
            match (message.role, part) {
                (
                    Role::Assistant,
                    ContentPart::ToolCall {
                        id,
                        name,
                        arguments,
                        ..
                    },
                ) => {
                    let input = serde_json::from_str(arguments)
                        .unwrap_or_else(|_| serde_json::Value::String(arguments.clone()));
                    self.pending.retain(|pending| pending.id != *id);
                    self.pending.push(Pending {
                        id: id.clone(),
                        signature: signature(name, &input),
                        tool: name.clone(),
                    });
                    if self.pending.len() > 128 {
                        self.pending.remove(0);
                    }
                }
                (
                    Role::Tool,
                    ContentPart::ToolResult {
                        tool_call_id,
                        content,
                    },
                ) => {
                    if let Some(index) = self
                        .pending
                        .iter()
                        .position(|pending| pending.id == *tool_call_id)
                    {
                        let pending = self.pending.remove(index);
                        self.result(pending, content, None, source_turn);
                    }
                }
                _ => {}
            }
        }
    }
    /// Record an explicit runtime outcome before an encoder drops its status.
    /// # Arguments
    /// `id` matches the pending call; `output`, `success` are the original result;
    /// `source_turn` identifies the result's future transcript position.
    /// # Returns
    /// Unit. A later message for the consumed call is ignored, avoiding double counting.
    /// # Examples
    /// ```
    /// let mut ledger = codetether_repeat_guard::ProgressLedger::default();
    /// ledger.observe_outcome("unknown", "output", true, 1);
    /// assert_eq!(ledger.observations, 0);
    /// ```
    pub fn observe_outcome(&mut self, id: &str, output: &str, success: bool, source_turn: usize) {
        if let Some(index) = self.pending.iter().position(|pending| pending.id == id) {
            let pending = self.pending.remove(index);
            self.result(pending, output, Some(success), source_turn);
        }
    }
}
