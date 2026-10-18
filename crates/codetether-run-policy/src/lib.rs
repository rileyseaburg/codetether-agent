//! Run configuration loading and session policy.
//!
//! [load] records effective access policy after applying CLI overrides.
//! [apply_session_policy] updates native sessions through [SessionPolicy].
mod load;
mod mode;
mod session;
pub use load::load;
pub use mode::effective_access_mode;
pub use session::apply_session_policy;
/// Minimal mutable session policy fields used by CLI runs.
/// # Implementors
/// Native sessions update their existing metadata and step-budget fields.
pub trait SessionPolicy {
    /// Assign the selected model to session metadata.
    fn set_model(&mut self, model: String);
    /// Enable automatic edit application.
    fn enable_auto_apply(&mut self);
    /// Set the optional step limit.
    fn set_max_steps(&mut self, max_steps: Option<usize>);
}

mod model;
pub use model::apply_model;
