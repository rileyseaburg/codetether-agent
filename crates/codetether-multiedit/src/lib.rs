//! Multi-file edits with validation before writing and optional Morph assistance.
//!
//! [MultiEditTool] receives its Morph editor from the caller. Parsing, content
//! caching, exact replacement, Morph fallback, writing and reporting live in
//! separate private modules. Multiple edits to one file use accumulated content.
use codetether_tool_core::{Tool, ToolResult};
mod apply;
mod exact;
mod execute;
mod input;
mod instruction;
mod load;
mod morph;
mod op;
mod op_fields;
mod op_file;
mod op_validate;
mod report;
mod schema;
mod tool;
mod update;
mod validate;
mod validated;
mod write;
/// Validate and apply multi-file edits using a caller-supplied Morph editor.
///
/// # Examples
/// ```rust
/// # fn example<E: codetether_morph_backend::Editor + Default>() {
/// let _tool = codetether_multiedit::MultiEditTool::<E>::new();
/// # }
/// ```
pub struct MultiEditTool<E> {
    editor: E,
}
impl<E: Default> MultiEditTool<E> {
    /// Create a tool with the editor's default configuration.
    ///
    /// # Arguments
    /// None; the editor is constructed through its `Default` implementation.
    /// # Returns
    /// A multi-edit tool bound to that editor.
    /// # Examples
    /// ```rust
    /// # fn example<E: codetether_morph_backend::Editor + Default>() {
    /// let _tool = codetether_multiedit::MultiEditTool::<E>::new();
    /// # }
    /// ```
    pub fn new() -> Self {
        Self {
            editor: E::default(),
        }
    }
}
impl<E: Default> Default for MultiEditTool<E> {
    fn default() -> Self {
        Self::new()
    }
}
