//! Read-only session views for context derivation and proactive preparation.
//! [crate::impl_session_context_views!] binds shared data access without native callbacks.
mod dispatch;
mod legacy;
mod proactive;
mod reset;
/// Add field-only context source contracts to a local session schema.
///
/// Requires the storage and persistence contracts for constraint reads and offsets.
/// Native preparation, model resolution and derivation callbacks remain separate.
/// # Usage
/// ```text
/// codetether_session_schema::impl_session_context_views!(Session);
/// ```
#[macro_export]
macro_rules! impl_session_context_views {
    ($name:ident) => {
        $crate::__session_context_legacy!($name);
        $crate::__session_context_reset!($name);
        $crate::__session_context_dispatch!($name);
        $crate::__session_context_proactive!($name);
    };
}
