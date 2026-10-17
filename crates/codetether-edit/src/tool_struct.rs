//! Edit tool construction and registry metadata.

use anyhow::Result;
use async_trait::async_trait;
use serde_json::Value;

use super::{execute, schema};
use codetether_tool_core::{Tool, ToolResult};

/// Prepare confirmation previews by replacing file text.
///
/// # Examples
/// ```rust
/// # fn example<E: codetether_morph_backend::Editor + Default>() {
/// let _tool = codetether_edit::EditTool::<E>::new();
/// # }
/// ```
pub struct EditTool<E> {
    editor: E,
}

impl<E: Default> Default for EditTool<E> {
    fn default() -> Self {
        Self::new()
    }
}

impl<E: Default> EditTool<E> {
    /// Create a preview tool with its default Morph editor.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool bound to the editor's default instance.
    /// # Examples
    /// ```rust
    /// # fn example<E: codetether_morph_backend::Editor + Default>() {
    /// let _tool = codetether_edit::EditTool::<E>::new();
    /// # }
    /// ```
    pub fn new() -> Self {
        Self {
            editor: E::default(),
        }
    }
}

#[async_trait]
impl<E: codetether_morph_backend::Editor> Tool for EditTool<E> {
    fn id(&self) -> &str {
        "edit"
    }

    fn name(&self) -> &str {
        "Edit File"
    }

    fn description(&self) -> &str {
        "edit(path, old_string, new_string, replace_all?) - Replace text. Falls back to whitespace-tolerant and fuzzy nearest-block matching."
    }

    fn parameters(&self) -> Value {
        schema::parameters()
    }

    async fn execute(&self, args: Value) -> Result<ToolResult> {
        execute::run(args, &self.editor).await
    }
}
