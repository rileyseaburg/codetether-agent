//! URL helpers for security metadata.

use super::Origin;

pub fn parse_origin(url: &str) -> Origin {
    let trimmed = url.trim();
    let Some((scheme, rest)) = trimmed.split_once("://") else {
        return Origin::Opaque(trimmed.split('#').next().unwrap_or(trimmed).into());
    };
    let authority = rest.split(['/', '?', '#']).next().unwrap_or_default();
    let host_port = authority
        .rsplit_once('@')
        .map_or(authority, |(_, host)| host);
    let Some((host, port)) = split_host_port(host_port) else {
        return Origin::Opaque(trimmed.split('#').next().unwrap_or(trimmed).into());
    };
    Origin::Tuple {
        scheme: scheme.to_ascii_lowercase(),
        host: host.to_ascii_lowercase(),
        port: normalize_port(scheme, port),
    }
}

fn split_host_port(authority: &str) -> Option<(&str, Option<u16>)> {
    if authority.is_empty() {
        return None;
    }
    if let Some((host, port)) = authority.rsplit_once(':') {
        if let Ok(port) = port.parse::<u16>() {
            return (!host.is_empty()).then_some((host, Some(port)));
        }
    }
    Some((authority, None))
}

fn normalize_port(scheme: &str, port: Option<u16>) -> Option<u16> {
    match (scheme.to_ascii_lowercase().as_str(), port) {
        ("http", Some(80)) | ("https", Some(443)) => None,
        _ => port,
    }
}
