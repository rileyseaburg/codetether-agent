//! Shared cursor delivery fixture.
use codetether_session_store::cursor_read;
pub fn next(path: &std::path::Path, name: &str) -> Option<codetether_session_store::Delivery<()>> {
    cursor_read::next(
        path,
        "constraints-session",
        name,
        24,
        |_| Ok(()),
        |_, _, _, _, _| Ok(()),
    )
    .unwrap()
}
