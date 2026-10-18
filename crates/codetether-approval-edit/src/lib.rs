//! In-memory editing of proposed patch contents before an approval decision.
//!
//! ApprovalEditSession loads proposed contents through the shared patch parser,
//! exposes one FileBuffer at a time, and emits a revised unified patch. It does
//! not write the revised contents or approve the pending request.
use std::path::PathBuf;
mod load;
mod patch;
mod session;
#[derive(Debug)]
struct ApprovalEditFile {
    path: PathBuf,
    relative: String,
    original: String,
    revised: String,
}

#[derive(Debug)]
/// Editable proposed file contents associated with one pending approval.
/// # Examples
/// ```no_run
/// let session = codetether_approval_edit::ApprovalEditSession::from_patch(
///     std::path::Path::new("."), "request-id".into(), "patch text").unwrap();
/// assert_eq!(session.id, "request-id");
/// ```
pub struct ApprovalEditSession {
    /// ID of the approval being edited.
    pub id: String,
    files: Vec<ApprovalEditFile>,
    index: usize,
}
