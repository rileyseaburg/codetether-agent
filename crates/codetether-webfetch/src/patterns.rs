//! patterns for web content fetching.
use super::*;

#[allow(dead_code)]
pub(super) static RE_STRIP_SCRIPT_STYLE: Lazy<Regex> = Lazy::new(|| {
    // DEPRECATED: kept for backwards compatibility with old builds.
    // NOTE: The Rust `regex` crate does NOT support backreferences, so we do
    // not rely on this regex for correctness. See `preprocess_html()`.
    Regex::new(r"(?is)<script[^>]*>.*?</script>").expect("invalid regex")
});

pub(super) static RE_STRIP_HTML_COMMENTS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<!--.*?-->").expect("invalid regex"));

pub(super) static RE_STRIP_SCRIPT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<script[^>]*>.*?</script>").expect("invalid regex"));

pub(super) static RE_STRIP_STYLE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<style[^>]*>.*?</style>").expect("invalid regex"));

pub(super) static RE_STRIP_NOSCRIPT: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<noscript[^>]*>.*?</noscript>").expect("invalid regex"));

pub(super) static RE_STRIP_SVG: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<svg[^>]*>.*?</svg>").expect("invalid regex"));

pub(super) static RE_STRIP_CANVAS: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<canvas[^>]*>.*?</canvas>").expect("invalid regex"));

pub(super) static RE_STRIP_IFRAME: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"(?is)<iframe[^>]*>.*?</iframe>").expect("invalid regex"));
