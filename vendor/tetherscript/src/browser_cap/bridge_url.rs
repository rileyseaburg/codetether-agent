//! HTTP bridge URL parser for the std-only browser transport.

pub(crate) struct BridgeUrl {
    pub(crate) host: String,
    pub(crate) port: u16,
    pub(crate) host_header: String,
    pub(crate) target: String,
}

#[path = "bridge_url_parts.rs"]
mod parts;

impl BridgeUrl {
    pub(crate) fn parse(input: &str) -> Result<Self, String> {
        let rest = input
            .strip_prefix("http://")
            .ok_or_else(|| "browser bridge endpoint must be http://".to_string())?;
        let split_at = rest
            .char_indices()
            .find_map(|(i, ch)| matches!(ch, '/' | '?').then_some(i))
            .unwrap_or(rest.len());
        let authority = &rest[..split_at];
        let suffix = &rest[split_at..];
        if authority.is_empty() {
            return Err("browser bridge URL missing host".into());
        }
        let (host, port) = parts::parse_authority(authority)?;
        let host_header = parts::host_header(&host, port);
        let target = parts::target_path(suffix);
        Ok(Self {
            host,
            port,
            host_header,
            target,
        })
    }
}
