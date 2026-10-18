//! Atomic snapshot bytes and verified no-op save detection.
//!
//! [atomic_write] replaces a file through a unique durable temporary file.
//! [record_saved] remembers successful bytes; [is_unchanged] also verifies the
//! current destination bytes before allowing a save to be skipped. [forget]
//! removes the remembered hash when a snapshot is deleted.
mod atomic;
mod guard;
pub use atomic::atomic_write;
pub use guard::{forget, is_unchanged, record_saved};

mod size;
pub use size::{MAX_SESSION_FILE_BYTES, ensure_session_file_size};
