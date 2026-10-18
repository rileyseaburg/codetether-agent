//! One-shot delivery of the collected goals.
use crate::GoalPromptState;
impl GoalPromptState {
    /// Send the collected goals back through the channel and consume it.
    pub fn finish(&mut self) {
        if let Some(tx) = self.responder.take() {
            let _ = tx.send(std::mem::take(&mut self.goals));
        }
    }
}
