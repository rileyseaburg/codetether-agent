//! Edit preview tool bound to the application-owned Morph backend.
/// Edit preview tool using the shared application provider registry.
///
/// # Examples
/// ```rust
/// let _tool = codetether_native_edit::edit::EditTool::new();
/// ```
pub type EditTool = codetether_edit::EditTool<super::morph_backend::ApplicationEditor>;
#[doc(hidden)]
pub use codetether_edit::proposed;
