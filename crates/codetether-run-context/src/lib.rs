//! Prepare file attachments and optional workspace knowledge for a run.
//!
//! [with_attachments] validates paths and adds them to the prompt.
//! [refresh_knowledge] applies the environment opt-in before refreshing a snapshot.
mod files;
mod knowledge;
pub use files::with_attachments;
pub use knowledge::refresh as refresh_knowledge;
