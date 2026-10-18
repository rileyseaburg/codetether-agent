//! Application execution choices for native forage services.
mod runtime;
pub(crate) type Native = codetether_native_forage::Host<runtime::Application>;
