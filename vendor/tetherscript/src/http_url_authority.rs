//! URL authority parsing for std HTTP.

use super::port::{default_port, host_header, port_number};

pub(crate) fn parse(authority: &str, https: bool) -> Result<(String, u16, String), String> {
    if authority.starts_with('[') {
        return super::ipv6::parse(authority, https);
    }
    if authority.matches(':').count() > 1 {
        return Err("http_request: IPv6 hosts must be bracketed".into());
    }
    let (host, port) = match authority.split_once(':') {
        Some((host, port)) => (host.to_string(), port_number(port)?),
        None => (authority.to_string(), default_port(https)),
    };
    validate_host(&host)?;
    Ok((host.clone(), port, host_header(&host, port, https, false)))
}

fn validate_host(host: &str) -> Result<(), String> {
    if host.is_empty() {
        return Err("http_request: missing URL host".into());
    }
    if host.chars().any(char::is_whitespace) {
        return Err("http_request: URL host must not contain whitespace".into());
    }
    Ok(())
}
