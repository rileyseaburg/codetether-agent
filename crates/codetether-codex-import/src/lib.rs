//! Import orchestration for Codex archives and native session storage.
//!
//! [Host] supplies native parsing, persistence and resume operations.
//! The import functions select archives, preserve indexed titles, persist
//! imported data and resume the stored session using the existing precedence.
mod by_id;
mod by_path;
mod directory;
mod host;
mod persist;
mod report;
mod resume;
pub use by_id::import_codex_session_by_id;
pub use by_path::import_codex_session_path;
pub use directory::import_codex_sessions_for_directory;
pub use host::Host;
pub use persist::persist_imported_session;
pub use report::{CodexImportReport, PersistOutcome};
pub use resume::load_or_import_session;
