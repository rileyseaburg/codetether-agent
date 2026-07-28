//! URL pattern matching for network routes.

/// URL matcher for a route rule.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::RoutePattern;
///
/// assert!(RoutePattern::glob("**/api/*").matches("https://x.test/api/items"));
/// assert!(RoutePattern::substring("/api/").matches("https://x.test/api/items"));
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RoutePattern {
    /// Match every URL.
    Any,
    /// Match when the URL contains the stored text.
    Substring(String),
    /// Match a glob-lite pattern where `*` means any byte sequence.
    Glob(String),
}

impl RoutePattern {
    /// Creates a substring URL pattern.
    pub fn substring(value: impl Into<String>) -> Self {
        Self::Substring(value.into())
    }

    /// Creates a glob-lite URL pattern.
    pub fn glob(value: impl Into<String>) -> Self {
        Self::Glob(value.into())
    }

    /// Returns whether the pattern matches `url`.
    pub fn matches(&self, url: &str) -> bool {
        match self {
            Self::Any => true,
            Self::Substring(value) => url.contains(value),
            Self::Glob(value) => glob_matches(value, url),
        }
    }
}

fn glob_matches(pattern: &str, url: &str) -> bool {
    let parts: Vec<&str> = pattern.split('*').collect();
    if parts.len() == 1 {
        return pattern == url;
    }
    let mut offset = 0;
    for (index, part) in parts.iter().enumerate() {
        if part.is_empty() {
            continue;
        }
        if index == 0 && !pattern.starts_with('*') && !url.starts_with(part) {
            return false;
        }
        let Some(found) = url[offset..].find(part) else {
            return false;
        };
        offset += found + part.len();
    }
    pattern.ends_with('*') || parts.last().is_some_and(|last| url.ends_with(last))
}
