//! Goal-list editing and empty-line completion detection.
use crate::GoalPromptState;
use tokio::sync::oneshot;
impl GoalPromptState {
    /// Create a new prompt with a responder channel.
    ///
    /// # Examples
    /// ```
    /// let (tx, _rx) = tokio::sync::oneshot::channel();
    /// let state = codetether_goal_prompt::GoalPromptState::new("Goals", "Add a goal", tx);
    /// assert!(state.current.is_empty());
    /// ```
    pub fn new(
        title: impl Into<String>,
        body: impl Into<String>,
        responder: oneshot::Sender<Vec<String>>,
    ) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            goals: Vec::new(),
            current: String::new(),
            responder: Some(responder),
        }
    }

    /// Append a typed character to the current line.
    pub fn push_char(&mut self, c: char) {
        self.current.push(c);
    }

    /// Remove the last character from the current line.
    pub fn backspace(&mut self) {
        self.current.pop();
    }

    /// Commit the current line as a goal, or signal "finished" if it's blank.
    ///
    /// Returns `true` when the user pressed Enter on an empty line (done).
    pub fn submit_line(&mut self) -> bool {
        let trimmed = self.current.trim().to_string();
        if trimmed.is_empty() {
            return true;
        }
        self.goals.push(trimmed);
        self.current.clear();
        false
    }
}
