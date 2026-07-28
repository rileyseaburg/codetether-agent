//! URL default ports.

pub(crate) fn default_port(https: bool) -> u16 {
    if https {
        443
    } else {
        80
    }
}

pub(crate) fn host_header(host: &str, port: u16, https: bool, ipv6: bool) -> String {
    let host = if ipv6 {
        format!("[{host}]")
    } else {
        host.into()
    };
    if port == default_port(https) {
        host
    } else {
        format!("{host}:{port}")
    }
}

pub(crate) fn port_number(port: &str) -> Result<u16, String> {
    if port.is_empty() {
        return Err("http_request: empty URL port".into());
    }
    let port = port
        .parse()
        .map_err(|_| format!("http_request: invalid URL port {port}"))?;
    if port == 0 {
        Err("http_request: URL port must be greater than zero".into())
    } else {
        Ok(port)
    }
}
