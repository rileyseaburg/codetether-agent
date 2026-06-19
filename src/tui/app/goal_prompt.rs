//! Friendly, non-technical goal-setup prompt state for the TUI.
//!
//! When forage (or any feature) finds nothing to work on, it opens this modal
//! instead of dead-ending. The user types plain-English goals one at a time;
//! the collected list is delivered back to the waiting task via a channel.

use tokio::sync::oneshot;

/// State for the active goal-setup modal.
///
/// While `Some(GoalPromptState)` is stored on the app, the event loop routes
/// all keystrokes here and the overlay renders the card. Submitting (or
/// skipping) sends the collected goals through `responder` and clears state.
pub struct GoalPromptState {
    /// Big friendly heading, e.g. "Let's set up your goals".
    pub title: String,
    /// Plain-English explanation shown under the title.
    pub body: String,
    /// Goals the user has added so far.
    pub goals: Vec<String>,
    /// The line currently being typed.
    pub current: String,
    /// Channel used to hand the result back to the waiting task.
    pub responder: Option<oneshot::Sender<Vec<String>>>,
}

impl GoalPromptState {
    /// Create a new prompt with a responder channel.
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

    /// Send the collected goals back through the channel and consume it.
    pub fn finish(&mut self) {
        if let Some(tx) = self.responder.take() {
            let _ = tx.send(std::mem::take(&mut self.goals));
        }
    }
}

#[cfg(test)]
#[path = "goal_prompt_tests.rs"]
mod tests;
