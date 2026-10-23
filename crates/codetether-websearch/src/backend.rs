//! Search backend identity, selection order, and request routing.
use crate::{SearchFailure, bing, duckduckgo, result::SearchResult};

/// Environment variable selecting backends: `auto` (default), `bing`, `duckduckgo`.
pub const BACKEND_ENV: &str = "CODETETHER_WEBSEARCH_BACKEND";

/// A concrete web-search provider contacted by [`crate::WebSearchTool`].
///
/// # Examples
/// ```rust
/// use codetether_websearch::{Backend, backend_order};
/// assert_eq!(backend_order(None).unwrap(), vec![Backend::Bing, Backend::DuckDuckGo]);
/// assert_eq!(backend_order(Some("bing")).unwrap(), vec![Backend::Bing]);
/// assert!(backend_order(Some("altavista")).is_err());
/// match Backend::Bing {
///     Backend::Bing => assert_eq!(Backend::Bing.id(), "bing"),
///     Backend::DuckDuckGo => unreachable!(),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// Bing HTML results (`www.bing.com/search`).
    Bing,
    /// DuckDuckGo HTML results (`html.duckduckgo.com/html`).
    DuckDuckGo,
}

impl Backend {
    /// Stable backend identifier recorded in search evidence.
    pub fn id(self) -> &'static str {
        match self {
            Self::Bing => "bing",
            Self::DuckDuckGo => "duckduckgo",
        }
    }

    /// Request URL and the response header carrying a provider request ID.
    pub(crate) fn request(self, query: &str) -> (String, Option<&'static str>) {
        match self {
            Self::Bing => (bing::url(query), Some("x-eventid")),
            Self::DuckDuckGo => (duckduckgo::url(query), None),
        }
    }

    /// Parse a provider page into results or a typed failure.
    pub(crate) fn parse(
        self,
        status: u16,
        body: &str,
        max: usize,
    ) -> Result<Vec<SearchResult>, SearchFailure> {
        match self {
            Self::Bing => bing::parse(status, body, max),
            Self::DuckDuckGo => duckduckgo::parse(status, body, max),
        }
    }
}

/// Resolve the ordered backend list from a [`BACKEND_ENV`] value.
///
/// `auto` tries Bing, then DuckDuckGo only if Bing fails; explicit values never
/// fall back. Every attempt is recorded in the search evidence line.
///
/// # Errors
/// Returns [`SearchFailure::Config`] for unsupported values.
pub fn backend_order(value: Option<&str>) -> Result<Vec<Backend>, SearchFailure> {
    match value.map(|v| v.trim().to_ascii_lowercase()).as_deref() {
        None | Some("" | "auto") => Ok(vec![Backend::Bing, Backend::DuckDuckGo]),
        Some("bing") => Ok(vec![Backend::Bing]),
        Some("duckduckgo" | "ddg") => Ok(vec![Backend::DuckDuckGo]),
        Some(other) => Err(SearchFailure::Config(format!(
            "{BACKEND_ENV}={other} is not one of auto, bing, duckduckgo"
        ))),
    }
}
