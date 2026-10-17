//! Read-only access to a validated request batch.
use super::{Question, Request};
impl Request {
    /// Borrow the validated questions in answer order.
    pub fn questions(&self) -> &[Question] {
        &self.questions
    }
}
