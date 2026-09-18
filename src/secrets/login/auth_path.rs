//! Validate non-administrator auth role and mount identifiers.
use anyhow::{Result, ensure};
pub(crate) fn auth_path(mount: &str, role: &str) -> Result<String> {
    ensure!(
        !mount.is_empty()
            && mount
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'-' | b'_')),
        "Invalid Vault auth mount"
    );
    ensure!(
        !role.is_empty()
            && role
                .bytes()
                .all(|c| c.is_ascii_alphanumeric() || matches!(c, b'-' | b'_')),
        "Invalid Vault role"
    );
    ensure!(
        !["admin", "superadmin", "root"].contains(&role.to_ascii_lowercase().as_str()),
        "Use a dedicated CodeTether role, never a Vault administrator role"
    );
    Ok(format!("auth/{mount}/login"))
}
