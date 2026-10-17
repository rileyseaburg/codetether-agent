//! tool support for the TetherScript plugin tool.
use std::path::{Path, PathBuf};

/// Plugin execution tool rooted at a workspace directory.
///
/// # Examples
/// ```rust
/// # fn example<H:codetether_tetherscript_tool::Host>() {
/// let _tool=codetether_tetherscript_tool::TetherScriptPluginTool::<H>::new();
/// # }
/// ```
pub struct TetherScriptPluginTool<H> {
    host: std::marker::PhantomData<H>,
    root: PathBuf,
}

impl<H: crate::Host> TetherScriptPluginTool<H> {
    /// Use the current directory as the plugin workspace root.
    ///
    /// # Examples
    /// ```rust
    /// # fn example<H:codetether_tetherscript_tool::Host>() {
    /// let _tool=codetether_tetherscript_tool::TetherScriptPluginTool::<H>::new();
    /// # }
    /// ```
    pub fn new() -> Self {
        Self {
            host: std::marker::PhantomData,
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    /// Use an explicit root for loading workspace-confined plugin files.
    ///
    /// # Examples
    /// ```rust
    /// # fn example<H:codetether_tetherscript_tool::Host>() {
    /// let _tool=codetether_tetherscript_tool::TetherScriptPluginTool::<H>::with_root(".".into());
    /// # }
    /// ```
    pub fn with_root(root: PathBuf) -> Self {
        Self {
            host: std::marker::PhantomData,
            root,
        }
    }

    pub(crate) fn root(&self) -> &Path {
        &self.root
    }
}

impl<H: crate::Host> Default for TetherScriptPluginTool<H> {
    fn default() -> Self {
        Self::new()
    }
}
