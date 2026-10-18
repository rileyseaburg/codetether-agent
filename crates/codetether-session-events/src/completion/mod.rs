//! Completion notices carrying an owned session payload.
//! [notice] converts application errors and caught panics into [SessionNotice]
//! without losing the session. Frontends choose the session payload type.
mod notice;
mod result;
pub use notice::SessionNotice;
pub use result::{PromptRunResult, notice};
