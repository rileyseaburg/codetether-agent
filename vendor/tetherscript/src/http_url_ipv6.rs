//! Bracketed IPv6 URL authority parsing.

use super::port::{default_port, host_header, port_number};

pub(crate) fn parse(authority: &str, https: bool) -> Result<(String, u16, String), String> {
    let end = authority
        .find(']')
        .ok_or_else(|| "http_request: invalid bracketed IPv6 host".to_string())?;
    let host = authority[1..end].to_string();
    if host.is_empty() {
        return Err("http_request: empty IPv6 host".into());
    }
    let rest = &authority[end + 1..];
    let port = if rest.is_empty() {
        default_port(https)
    } else {
        port_number(port_suffix(rest)?)?
    };
    Ok((host.clone(), port, host_header(&host, port, https, true)))
}

fn port_suffix(rest: &str) -> Result<&str, String> {
    rest.strip_prefix(':')
        .ok_or_else(|| "http_request: invalid authority after IPv6 host".into())
}
