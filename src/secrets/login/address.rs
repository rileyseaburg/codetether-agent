//! Reject credential-bearing URLs and insecure non-loopback Vault endpoints.

use anyhow::{Result, ensure};

pub(crate) fn normalize(value: &str) -> Result<String> {
    let url = reqwest::Url::parse(value).map_err(|_| anyhow::anyhow!("Invalid Vault URL"))?;
    let loopback = url.host_str().is_some_and(|host| {
        host == "localhost"
            || host
                .trim_matches(['[', ']'])
                .parse::<std::net::IpAddr>()
                .is_ok_and(|ip| ip.is_loopback())
    });
    ensure!(
        url.scheme() == "https" || (url.scheme() == "http" && loopback),
        "Vault requires HTTPS (HTTP is allowed only on loopback)"
    );
    ensure!(
        url.username().is_empty()
            && url.password().is_none()
            && url.query().is_none()
            && url.fragment().is_none(),
        "Vault URL must not contain credentials, a query, or a fragment"
    );
    Ok(url.as_str().trim_end_matches('/').to_owned())
}
