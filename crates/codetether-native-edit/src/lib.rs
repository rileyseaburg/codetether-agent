//! Provider-bound single-file and multi-file editing.
//!
//! [edit::EditTool] and [multiedit::MultiEditTool] share the native provider
//! registry through [morph_backend::ApplicationEditor]. The backend preserves
//! provider selection and the unit-like editor construction used by callers.
pub mod edit;
pub mod morph_backend;
pub mod multiedit;

/// Session metadata and environment policy for automatic edit confirmation.
pub mod auto_apply;
/// Runtime-authorized undo using the shared application policy.
pub mod undo;
