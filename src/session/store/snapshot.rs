//! Bind shared persistence contracts to the native session type.
use crate::session::Session;
codetether_session_schema::impl_session_store!(Session);
