//! Multi-edit tool bound to the application-owned Morph provider resolver.
/// Multi-file editor using the application Morph backend.
///
/// # Examples
/// ```rust
/// let _tool = codetether_native_edit::multiedit::MultiEditTool::new();
/// ```
pub type MultiEditTool =
    codetether_multiedit::MultiEditTool<super::morph_backend::ApplicationEditor>;
