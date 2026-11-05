//! Validate token fields and prevent OAuth scope escalation.
use anyhow::{Result, ensure};

pub(super) fn valid_secret(value: &str) -> bool {
    !value.is_empty() && value.len() <= 32_768 && value.bytes().all(|b| b.is_ascii_graphic())
}

pub(super) fn validate_scope(scope: &str, allowed: &str) -> Result<()> {
    let scopes: Vec<_> = scope.split_whitespace().collect();
    ensure!(
        scopes.contains(&"mcp:read")
            && scopes
                .iter()
                .all(|s| allowed.split_whitespace().any(|a| a == *s)),
        "Unexpected Monarch token scope"
    );
    Ok(())
}
