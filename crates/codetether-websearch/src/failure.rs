//! Typed provider failures for web search.
//!
//! Every variant maps to a stable `WEBSEARCH_*` code so callers can tell a
//! blocked or broken provider apart from a search that legitimately matched
//! nothing (a success with zero results, never a failure).

/// Why a search backend could not produce a trustworthy result list.
///
/// # Examples
/// ```rust
/// use codetether_websearch::SearchFailure;
/// let failure = SearchFailure::Challenge { status: 202 };
/// assert_eq!(failure.code(), "WEBSEARCH_PROVIDER_CHALLENGE");
/// match failure {
///     SearchFailure::Challenge { status } => assert_eq!(status, 202),
///     other => panic!("unexpected {other:?}"),
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SearchFailure {
    /// `CODETETHER_WEBSEARCH_BACKEND` holds an unsupported value.
    Config(String),
    /// DNS, TLS, timeout, or connection failure before a usable response.
    Transport(String),
    /// The provider answered 401 or 403.
    Forbidden { status: u16 },
    /// The provider answered 429.
    RateLimited { status: u16 },
    /// The provider served a bot/anomaly challenge page instead of results.
    Challenge { status: u16 },
    /// Any other non-2xx status.
    HttpStatus { status: u16 },
    /// A 2xx page matching neither a result list nor a no-results page.
    Malformed(String),
}

impl SearchFailure {
    /// Stable machine-readable failure code.
    pub fn code(&self) -> &'static str {
        match self {
            Self::Config(_) => "WEBSEARCH_CONFIG_INVALID",
            Self::Transport(_) => "WEBSEARCH_TRANSPORT_FAILED",
            Self::Forbidden { .. } => "WEBSEARCH_PROVIDER_FORBIDDEN",
            Self::RateLimited { .. } => "WEBSEARCH_RATE_LIMITED",
            Self::Challenge { .. } => "WEBSEARCH_PROVIDER_CHALLENGE",
            Self::HttpStatus { .. } => "WEBSEARCH_HTTP_ERROR",
            Self::Malformed(_) => "WEBSEARCH_MALFORMED_RESPONSE",
        }
    }

    /// Human-readable detail for logs and tool output.
    pub fn detail(&self) -> String {
        match self {
            Self::Config(text) | Self::Transport(text) | Self::Malformed(text) => text.clone(),
            Self::Forbidden { status }
            | Self::RateLimited { status }
            | Self::HttpStatus { status } => format!("provider returned HTTP {status}"),
            Self::Challenge { status } => {
                format!("provider returned a bot challenge page (HTTP {status})")
            }
        }
    }

    /// Classify a non-success HTTP status; `None` for 2xx.
    pub(crate) fn from_status(status: u16) -> Option<Self> {
        match status {
            200..=299 => None,
            401 | 403 => Some(Self::Forbidden { status }),
            429 => Some(Self::RateLimited { status }),
            _ => Some(Self::HttpStatus { status }),
        }
    }
}
