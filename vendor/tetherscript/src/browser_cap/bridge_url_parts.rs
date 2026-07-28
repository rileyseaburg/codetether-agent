//! Small helpers for bridge URL parsing.

pub(crate) fn parse_authority(authority: &str) -> Result<(String, u16), String> {
    match authority.rsplit_once(':') {
        Some((h, p)) if p.chars().all(|c| c.is_ascii_digit()) => Ok((
            h.to_string(),
            p.parse()
                .map_err(|_| "browser bridge bad port".to_string())?,
        )),
        _ => Ok((authority.to_string(), 80)),
    }
}

pub(crate) fn host_header(host: &str, port: u16) -> String {
    if port == 80 {
        host.into()
    } else {
        format!("{}:{}", host, port)
    }
}

pub(crate) fn target_path(suffix: &str) -> String {
    if suffix.is_empty() {
        "/".into()
    } else if suffix.starts_with('?') {
        format!("/{}", suffix)
    } else {
        suffix.into()
    }
}
