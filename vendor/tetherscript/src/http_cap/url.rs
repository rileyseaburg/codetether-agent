pub(super) struct ParsedUrl {
    scheme: String,
    host: String,
    port: Option<u16>,
    pub(super) path: String,
}

impl ParsedUrl {
    pub(super) fn parse(url: &str) -> Result<Self, String> {
        let (scheme, rest) = url
            .split_once("://")
            .ok_or_else(|| format!("http: url `{}` has no scheme", url))?;
        let scheme = scheme.to_ascii_lowercase();
        if scheme != "http" && scheme != "https" {
            return Err(format!("http: scheme `{}` not supported", scheme));
        }
        let (authority, path) = match rest.find('/') {
            Some(index) => (&rest[..index], &rest[index..]),
            None => (rest, "/"),
        };
        let (host, port) = parse_authority(authority, url)?;
        Ok(Self {
            scheme,
            host,
            port,
            path: path.to_string(),
        })
    }

    pub(super) fn origin(&self) -> String {
        match self.port {
            Some(port) => format!("{}://{}:{}", self.scheme, self.host, port),
            None => format!("{}://{}", self.scheme, self.host),
        }
    }
}

pub(super) fn normalize_origin(origin: String) -> String {
    let trimmed = origin.trim_end_matches('/').to_string();
    ParsedUrl::parse(&trimmed)
        .map(|parsed| parsed.origin())
        .unwrap_or(trimmed)
}

fn parse_authority(authority: &str, url: &str) -> Result<(String, Option<u16>), String> {
    match authority.rsplit_once(':') {
        Some((host, port)) => Ok((
            host.to_ascii_lowercase(),
            Some(port.parse().map_err(|_| format!("http: bad port in `{}`", url))?),
        )),
        None => Ok((authority.to_ascii_lowercase(), None)),
    }
}
