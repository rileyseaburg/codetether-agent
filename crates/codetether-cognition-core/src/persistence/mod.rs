//! Saved cognition state, evidence selection and file persistence.
//!
//! [save_state] captures stores and writes atomically via a temporary path.
//! [load_state] guards file size and schema, reporting [LoadEvent] diagnostics
//! to the host. Application directory selection is supplied by the caller.
mod capture;
mod decode;
mod diagnostic;
mod evidence;
mod guard;
mod load;
mod references;
mod save;
mod state;
mod trim;
pub use diagnostic::LoadEvent;
pub use load::load_state;
pub use save::save_state;
pub use state::PersistedCognitionState;
#[cfg(test)]
mod tests;
