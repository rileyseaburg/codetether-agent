//! Shared title methods using the native provider registry.
use super::types::Session;
pub(super) use codetether_session_title as display;
pub(crate) use codetether_session_title::is_title_candidate;
codetether_session_schema::impl_session_titles!(Session, crate::provider::ProviderRegistry);
