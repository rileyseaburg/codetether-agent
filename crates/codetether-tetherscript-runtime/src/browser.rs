//! Browser capability grant settings.
/// Browser capability grant parameters passed from tool input.
///
/// # Examples
/// ```rust
/// let grant=codetether_tetherscript_runtime::BrowserGrant{endpoint:None,origins:vec![],scopes:vec![]};
/// assert!(grant.endpoint.is_none());
/// ```
pub struct BrowserGrant {
    pub endpoint: Option<String>,
    pub origins: Vec<String>,
    pub scopes: Vec<String>,
}
