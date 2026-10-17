//! Navigation and text/selection editing transitions for a question draft.
use super::{Action, Draft};
use crate::Answers;
impl Draft {
    /// Apply an action, returning ordered answers only on final submission.
    /// Empty custom answers and empty multi-selections keep the question open.
    ///
    /// # Examples
    /// ```
    /// use codetether_question::{Action, Draft, Request};
    /// let mut d = Draft::new(Request::parse(serde_json::json!({"question":"Why?"})).unwrap());
    /// assert!(d.apply(Action::Submit).is_none());
    /// d.apply(Action::Text("Because".into()));
    /// assert_eq!(d.apply(Action::Submit).unwrap()[0], ["Because"]);
    /// ```
    pub fn apply(&mut self, action: Action) -> Option<Answers> {
        let count = self.question().options.len();
        let cursor = self.cursor();
        match action {
            Action::Next => self.cursors[self.index] = (cursor + 1) % (count + 1),
            Action::Previous => self.cursors[self.index] = (cursor + count) % (count + 1),
            Action::Toggle if self.question().multiple && cursor < count => {
                self.selected[self.index][cursor] ^= true;
            }
            Action::Text(text) => {
                self.cursors[self.index] = count;
                self.custom[self.index].push_str(&text);
            }
            Action::Backspace => {
                self.custom[self.index].pop();
            }
            Action::Back if self.index > 0 => {
                self.index -= 1;
            }
            Action::Submit => {
                return self.submit(count, cursor);
            }
            Action::Toggle | Action::Back => {}
        }
        None
    }
}
