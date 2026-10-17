//! Edit preview tool bound to the application-owned Morph backend.
/// Edit preview tool using the shared application provider registry.
///
/// # Examples
/// ```rust
/// let _tool = codetether_agent::tool::edit::EditTool::new();
/// ```
pub type EditTool = codetether_edit::EditTool<super::morph_backend::ApplicationEditor>;
pub(crate) use codetether_edit::proposed;
