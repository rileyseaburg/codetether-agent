use super::cookies::{Cookie, valid_name, valid_value};
use anyhow::{Result, ensure};
pub(super) fn record(f: &[&str]) -> Result<Cookie> {
    ensure!(f.len() == 7, "cookie_domain_metadata_required");
    let domain = f[0].strip_prefix('.').unwrap_or(f[0]).to_ascii_lowercase();
    ensure!(
        domain.contains('.')
            && domain.split('.').all(|part| !part.is_empty()
                && !part.starts_with('-')
                && !part.ends_with('-')
                && part.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'-')),
        "cookie_domain_invalid"
    );
    ensure!(
        matches!(f[1], "TRUE" | "FALSE") && matches!(f[3], "TRUE" | "FALSE"),
        "cookie_flags_invalid"
    );
    ensure!(
        f[2].starts_with('/') && !f[2].bytes().any(|b| b.is_ascii_control()),
        "cookie_path_invalid"
    );
    ensure!(
        valid_name(f[5]) && valid_value(f[6]),
        "cookie_value_invalid"
    );
    let expires = f[4]
        .parse::<u64>()
        .map_err(|_| anyhow::anyhow!("cookie_expiry_invalid"))?;
    Ok(Cookie {
        domain,
        subdomains: f[1] == "TRUE",
        path: f[2].into(),
        secure: f[3] == "TRUE",
        expires,
        name: f[5].into(),
        value: f[6].into(),
    })
}
