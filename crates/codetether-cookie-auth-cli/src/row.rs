//! Cookie import row responsibilities.

#[derive(Debug, Clone)]
pub(super) struct CookieRow {
    pub(super) domain: String,
    pub(super) include_subdomains: bool,
    pub(super) path: String,
    pub(super) secure: bool,
    pub(super) expires_epoch: i64,
    pub(super) name: String,
    pub(super) value: String,
    pub(super) http_only: bool,
}
