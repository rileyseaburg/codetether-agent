//! Native session projection for shared resume status text.
use crate::session::Session;
pub fn load_status_with_original(
    session: &Session,
    dropped: usize,
    file_bytes: u64,
    original_id: Option<&str>,
) -> String {
    let label = session.display_label();
    codetether_session_resume::load_status_with_original(
        &label,
        session.messages.len(),
        dropped,
        file_bytes,
        original_id,
    )
}
