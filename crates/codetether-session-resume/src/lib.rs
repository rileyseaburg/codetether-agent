//! Session-resume outcome metadata, diagnostics and bounded-window policy.
//!
//! Hosts load and fork sessions, then construct SessionLoadOutcome. Its status,
//! banner and history_source methods describe the result without owning sessions.
mod absence;
mod banner;
pub mod continuation;
mod load_status;
pub mod loading;
mod outcome;
mod status;
mod window;
pub use absence::is_expected_absence;
pub use load_status::load_status_with_original;
pub use outcome::SessionLoadOutcome;
pub use window::session_resume_window;

/// Interactive session selection and workspace import orchestration.
pub mod interactive;
