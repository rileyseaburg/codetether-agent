//! Shared stored-paste fixture for regression tests.
use crate::PendingTextPaste;

pub(super) fn paste(id: u32, content: &str) -> PendingTextPaste {
    PendingTextPaste {
        id,
        content: content.to_string(),
    }
}
