use std::future::Future;
/// Goal and approval handlers that can consume input before prompt capture.
/// # Implementors
/// Native UI state supplies the three ordered handlers.
pub trait Intercept {
    /// Process explicit goal-answer commands; true consumes the input.
    fn goal_command(&mut self) -> impl Future<Output = bool>;
    /// Process an ordinary answer to a pending goal question.
    fn goal_answer(&mut self) -> impl Future<Output = bool>;
    /// Process feedback for pending approval.
    fn approval_feedback(&mut self) -> bool;
}
