//! Tool construction and workspace cache keys.
use super::{Host, LspTool};
impl<H: Host> LspTool<H> {
    /// Create a tool with default root and LSP settings.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// A tool using the host's shared manager cache.
    /// # Examples
    /// ```rust
    /// # fn example<H: codetether_lsp_tool::Host>() {
    /// let _tool = codetether_lsp_tool::LspTool::<H>::new();
    /// # }
    /// ```
    pub fn new() -> Self {
        Self {
            host: std::marker::PhantomData,
            root_uri: None,
            lsp_settings: None,
        }
    }

    /// Create with a specific workspace root
    ///
    /// # Arguments
    /// `root_uri` identifies the workspace.
    /// # Returns
    /// A tool using the supplied root.
    /// # Examples
    /// ```rust
    /// # fn example<H: codetether_lsp_tool::Host>() {
    /// let _tool = codetether_lsp_tool::LspTool::<H>::with_root("file:///workspace".into());
    /// # }
    /// ```
    pub fn with_root(root_uri: String) -> Self {
        Self {
            host: std::marker::PhantomData,
            root_uri: Some(root_uri),
            lsp_settings: None,
        }
    }

    /// Create with workspace root and config-driven LSP settings.
    ///
    /// # Arguments
    /// `root_uri` identifies the workspace; `settings` supplies explicit server configuration.
    /// # Returns
    /// A tool with these settings and the host's shared cache.
    /// # Examples
    /// ```rust
    /// # fn example<H: codetether_lsp_tool::Host>() {
    /// let _tool = codetether_lsp_tool::LspTool::<H>::with_config(None, Default::default());
    /// # }
    /// ```
    pub fn with_config(root_uri: Option<String>, settings: codetether_config::LspSettings) -> Self {
        Self {
            host: std::marker::PhantomData,
            root_uri,
            lsp_settings: Some(settings),
        }
    }

    pub(super) fn manager_key(&self) -> String {
        self.root_uri
            .clone()
            .unwrap_or_else(|| "__default__".to_string())
    }
}
impl<H: Host> Default for LspTool<H> {
    fn default() -> Self {
        Self::new()
    }
}
