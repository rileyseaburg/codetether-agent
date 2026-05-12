/// Browser capability grant parameters passed from tool input.
pub struct BrowserGrant {
    pub endpoint: Option<String>,
    pub origins: Vec<String>,
    pub scopes: Vec<String>,
}
