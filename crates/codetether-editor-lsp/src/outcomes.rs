//! Definition and hover request outcomes for a frontend to apply.
use codetether_lsp_core::LocationInfo;
/// First definition location, or explanatory status when no location is usable.
/// # Examples
/// ```
/// use codetether_editor_lsp::DefinitionOutcome;
/// let value = DefinitionOutcome::Status("indexing".into());
/// match value { DefinitionOutcome::Status(text) => assert_eq!(text, "indexing"), DefinitionOutcome::Location(_) => unreachable!() }
/// ```
pub enum DefinitionOutcome {
    /// Destination returned by the language server.
    Location(LocationInfo),
    /// Lookup error or exhausted retries.
    Status(String),
}
/// Hover markdown, or explanatory status for an empty/failed lookup.
/// # Examples
/// ```
/// use codetether_editor_lsp::HoverOutcome;
/// let value = HoverOutcome::Content("docs".into());
/// match value { HoverOutcome::Content(text) => assert_eq!(text, "docs"), HoverOutcome::Status(_) => unreachable!() }
/// ```
pub enum HoverOutcome {
    /// Nonempty server-provided hover content.
    Content(String),
    /// Empty-result or error status.
    Status(String),
}
