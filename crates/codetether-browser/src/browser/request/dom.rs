//! DOM request payloads grouped by interaction responsibility.
//!
//! Selector scopes, editable inputs, and text-targeted controls share the
//! existing request API through these re-exports.
mod input;
mod selector;
mod text;
pub use input::{FillRequest, KeyPressRequest, TypeRequest, UploadRequest};
pub use selector::{ScopeRequest, SelectorRequest};
pub use text::{ClickTextRequest, ToggleRequest};

#[cfg(test)]
mod tests;
