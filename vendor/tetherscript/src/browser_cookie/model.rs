//! Cookie data model.

/// SameSite metadata for deterministic request-cookie decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SameSite {
    /// Send the cookie only for same-site modeled requests.
    Lax,
    /// Send the cookie only for strict same-site modeled requests.
    Strict,
    /// Send the cookie without same-site filtering.
    None,
}

impl Default for SameSite {
    fn default() -> Self {
        Self::Lax
    }
}

impl SameSite {
    pub(crate) fn parse(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "strict" => Self::Strict,
            "none" => Self::None,
            _ => Self::Lax,
        }
    }
}

/// One cookie stored in a deterministic browser cookie jar.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: String,
    pub path: String,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: SameSite,
    pub expires_at: Option<i64>,
    pub host_only: bool,
}

impl Cookie {
    pub(crate) fn is_expired(&self, now: i64) -> bool {
        self.expires_at.is_some_and(|expires| expires <= now)
    }

    pub(crate) fn same_key(&self, other: &Self) -> bool {
        self.name == other.name && self.domain == other.domain && self.path == other.path
    }
}
