//! Conversation continuity for inbound A2A requests.
//!
//! [resolve_session] validates context identifiers and uses [ConversationStore]
//! to resume persisted sessions. Failed loads of existing state remain errors.
mod resolve;
mod store;
mod validate;
pub use resolve::resolve_session;
pub use store::ConversationStore;
pub use validate::usable_as_session_id;
