//! Shared session lifecycle with the native provider registry binding.
use super::types::Session;
codetether_session_schema::impl_session_lifecycle!(Session);
codetether_session_schema::impl_session_models!(Session, crate::provider::ProviderRegistry);
