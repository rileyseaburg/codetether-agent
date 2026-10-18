//! Shared title methods using the native provider registry.
use super::types::Session;
codetether_session_schema::impl_session_titles!(Session, crate::provider::ProviderRegistry);
