//! Validate the loopback HTTP envelope and reject ambiguous query parameters.
use anyhow::{Result, anyhow, ensure};
use std::collections::HashMap;
use url::Url;

pub(super) fn parameters(request: &str, authority: &str) -> Result<HashMap<String, String>> {
    let mut lines = request.split("\r\n");
    let parts: Vec<_> = lines
        .next()
        .unwrap_or_default()
        .split_whitespace()
        .collect();
    ensure!(
        parts.len() == 3 && parts[0] == "GET" && parts[2] == "HTTP/1.1",
        "Invalid callback request"
    );
    ensure!(
        parts[1].starts_with("/callback?") && !parts[1].contains('#'),
        "Invalid callback path"
    );
    let hosts: Vec<_> = lines
        .filter_map(|line| line.split_once(':'))
        .filter(|(name, _)| name.eq_ignore_ascii_case("host"))
        .collect();
    ensure!(
        hosts.len() == 1 && hosts[0].1.trim() == authority,
        "Invalid callback host"
    );
    let url = Url::parse(&format!("http://{authority}{}", parts[1]))
        .map_err(|_| anyhow!("Invalid callback URL"))?;
    let mut values = HashMap::new();
    for (key, value) in url.query_pairs() {
        ensure!(
            values
                .insert(key.into_owned(), value.into_owned())
                .is_none(),
            "Duplicate callback parameter"
        );
    }
    Ok(values)
}
