//! Origin identity for same-origin checks.

use super::url::parse_origin;

/// Normalized browser origin identity.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::Origin;
///
/// let origin = Origin::parse("HTTPS://Example.test:443/path");
/// assert_eq!(origin.serialized(), "https://example.test");
/// ```
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Origin {
    /// Tuple origin with scheme, host, and optional non-default port.
    Tuple {
        /// Lowercase scheme.
        scheme: String,
        /// Lowercase host.
        host: String,
        /// Explicit non-default port.
        port: Option<u16>,
    },
    /// Opaque origin used for non-hierarchical URLs.
    Opaque(String),
}

impl Origin {
    /// Parse an origin from a URL-like string.
    pub fn parse(url: impl AsRef<str>) -> Self {
        parse_origin(url.as_ref())
    }

    /// Return the canonical serialized origin key.
    pub fn serialized(&self) -> String {
        match self {
            Self::Tuple { scheme, host, port } => match port {
                Some(port) => format!("{scheme}://{host}:{port}"),
                None => format!("{scheme}://{host}"),
            },
            Self::Opaque(value) => value.clone(),
        }
    }

    /// Return true when two origins are equal and non-opaque.
    pub fn is_same_origin(&self, other: &Self) -> bool {
        !self.is_opaque() && self == other
    }

    /// Return true for opaque origins.
    pub fn is_opaque(&self) -> bool {
        matches!(self, Self::Opaque(_))
    }
}
