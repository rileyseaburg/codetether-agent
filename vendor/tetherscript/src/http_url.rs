//! HTTP URL parsing for `http://` and `https://` schemes.

#[path = "http_url_authority.rs"]
mod authority;
#[path = "http_url_ipv6.rs"]
mod ipv6;
#[path = "http_url_port.rs"]
mod port;

/// Parsed components of an `http://` URL.
#[derive(Debug, PartialEq)]
pub(crate) struct ParsedHttpUrl {
    pub https: bool,
    pub host: String,
    pub port: u16,
    pub host_header: String,
    pub target: String,
}

impl ParsedHttpUrl {
    /// Parse an `http://` URL into its components.
    ///
    /// # Errors
    ///
    /// Returns a descriptive error for invalid URLs, missing hosts, bad ports,
    /// and other malformed input.
    pub(crate) fn parse(input: &str) -> Result<Self, String> {
        let (rest, https) = strip_scheme(input)?;
        if rest.is_empty() {
            return Err("http_request: missing URL authority".into());
        }
        if rest.contains('#') {
            return Err("http_request: URL fragments are not sent over HTTP".into());
        }

        let split_at = rest
            .char_indices()
            .find_map(|(idx, ch)| matches!(ch, '/' | '?').then_some(idx))
            .unwrap_or(rest.len());
        let authority = &rest[..split_at];
        let suffix = &rest[split_at..];
        if authority.is_empty() {
            return Err("http_request: missing URL host".into());
        }
        if authority.contains('@') {
            return Err("http_request: userinfo in URLs is not supported".into());
        }

        let (host, port, host_header) = authority::parse(authority, https)?;
        let target = if suffix.is_empty() {
            "/".to_string()
        } else if suffix.starts_with('?') {
            format!("/{}", suffix)
        } else {
            suffix.to_string()
        };

        Ok(Self {
            https,
            host,
            port,
            host_header,
            target,
        })
    }
}

fn strip_scheme(input: &str) -> Result<(&str, bool), String> {
    if let Some(rest) = input.strip_prefix("https://") {
        Ok((rest, true))
    } else {
        input
            .strip_prefix("http://")
            .map(|rest| (rest, false))
            .ok_or_else(|| "http_request: URL must start with http:// or https://".to_string())
    }
}
