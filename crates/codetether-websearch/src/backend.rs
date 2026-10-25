//! Search backend identity, selection order, and request routing.
use crate::{SearchFailure, bing, duckduckgo, result::SearchResult, searxng};

/// Environment variable selecting backends: `auto` (default), `searxng`,
/// `bing`, `duckduckgo`.
pub const BACKEND_ENV: &str = "CODETETHER_WEBSEARCH_BACKEND";

/// A concrete web-search provider contacted by [`crate::WebSearchTool`].
///
/// # Examples
/// ```rust
/// use codetether_websearch::{Backend, backend_order};
/// assert_eq!(backend_order(None, false).unwrap(), vec![Backend::Bing, Backend::DuckDuckGo]);
/// assert_eq!(backend_order(None, true).unwrap()[0], Backend::SearXng);
/// assert_eq!(backend_order(Some("bing"), true).unwrap(), vec![Backend::Bing]);
/// assert!(backend_order(Some("altavista"), false).is_err());
/// match Backend::Bing {
///     Backend::Bing => assert_eq!(Backend::Bing.id(), "bing"),
///     Backend::DuckDuckGo | Backend::SearXng => unreachable!(),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Backend {
    /// Self-hosted SearXNG JSON API at [`crate::SEARXNG_URL_ENV`].
    SearXng,
    /// Bing HTML results (`www.bing.com/search`).
    Bing,
    /// DuckDuckGo HTML results (`html.duckduckgo.com/html`).
    DuckDuckGo,
}

impl Backend {
    /// Stable backend identifier recorded in search evidence.
    pub fn id(self) -> &'static str {
        match self {
            Self::SearXng => "searxng",
            Self::Bing => "bing",
            Self::DuckDuckGo => "duckduckgo",
        }
    }

    /// Request URL and the response header carrying a provider request ID.
    pub(crate) fn request(
        self,
        query: &str,
    ) -> Result<(String, Option<&'static str>), SearchFailure> {
        match self {
            Self::SearXng => Ok((searxng::url(&searxng::base_url()?, query), None)),
            Self::Bing => Ok((bing::url(query), Some("x-eventid"))),
            Self::DuckDuckGo => Ok((duckduckgo::url(query), None)),
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
            Self::SearXng => searxng::parse(status, body, max),
            Self::Bing => bing::parse(status, body, max),
            Self::DuckDuckGo => duckduckgo::parse(status, body, max),
        }
    }
}

/// Resolve the ordered backend list from a [`BACKEND_ENV`] value.
///
/// `auto` tries SearXNG first when `searxng_configured` (i.e.
/// [`crate::SEARXNG_URL_ENV`] is set), then Bing, then DuckDuckGo, stopping
/// at the first success. Explicit values never fall back. Every attempt is
/// recorded in the search evidence line.
///
/// # Errors
/// Returns [`SearchFailure::Config`] for unsupported values.
pub fn backend_order(
    value: Option<&str>,
    searxng_configured: bool,
) -> Result<Vec<Backend>, SearchFailure> {
    let auto = match searxng_configured {
        true => vec![Backend::SearXng, Backend::Bing, Backend::DuckDuckGo],
        false => vec![Backend::Bing, Backend::DuckDuckGo],
    };
    match value.map(|v| v.trim().to_ascii_lowercase()).as_deref() {
        None | Some("" | "auto") => Ok(auto),
        Some("searxng") => Ok(vec![Backend::SearXng]),
        Some("bing") => Ok(vec![Backend::Bing]),
        Some("duckduckgo" | "ddg") => Ok(vec![Backend::DuckDuckGo]),
        Some(other) => Err(SearchFailure::Config(format!(
            "{BACKEND_ENV}={other} is not one of auto, searxng, bing, duckduckgo"
        ))),
    }
}
