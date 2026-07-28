//! Minimal URL origin parsing used by browser capability guards.

pub(crate) struct ParsedUrl {
    scheme: String,
    host: String,
    port: Option<u16>,
    pub(crate) path: String,
}

impl ParsedUrl {
    pub(crate) fn parse(url: &str) -> Result<Self, String> {
        let (scheme, rest) = url
            .split_once("://")
            .ok_or_else(|| format!("browser: url `{}` has no scheme", url))?;
        let scheme = scheme.to_ascii_lowercase();
        if scheme != "http" && scheme != "https" {
            return Err(format!("browser: unsupported url scheme `{}`", scheme));
        }
        let (authority, path) = match rest.find('/') {
            Some(i) => (&rest[..i], &rest[i..]),
            None => (rest, "/"),
        };
        let (host, port) = parse_authority(authority)?;
        Ok(Self {
            scheme,
            host,
            port,
            path: path.into(),
        })
    }

    pub(crate) fn origin(&self) -> String {
        match self.port {
            Some(p) => format!("{}://{}:{}", self.scheme, self.host, p),
            None => format!("{}://{}", self.scheme, self.host),
        }
    }
}

pub(crate) fn normalize_origin(s: String) -> String {
    ParsedUrl::parse(s.trim_end_matches('/'))
        .map(|p| p.origin())
        .unwrap_or(s)
}

fn parse_authority(authority: &str) -> Result<(String, Option<u16>), String> {
    match authority.rsplit_once(':') {
        Some((h, p)) if p.chars().all(|c| c.is_ascii_digit()) => Ok((
            h.to_ascii_lowercase(),
            Some(p.parse().map_err(|_| "browser: bad port".to_string())?),
        )),
        _ => Ok((authority.to_ascii_lowercase(), None)),
    }
}
