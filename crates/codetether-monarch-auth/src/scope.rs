//! Fail closed on stored grants broader than the built-in connector requires.
use anyhow::{Result, ensure};

pub(crate) fn require_read_only(scope: &str) -> Result<()> {
    ensure!(
        scope.split_whitespace().eq(["mcp:read"]),
        "Monarch tool access requires a read-only grant; run codetether auth monarch logout then codetether auth monarch login"
    );
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn stored_grants_must_be_read_only() {
        assert!(require_read_only("mcp:read").is_ok());
        assert!(require_read_only(" mcp:read ").is_ok());
        for scope in [
            "",
            "mcp:write",
            "mcp:read mcp:write",
            "mcp:read unknown",
            "mcp:read mcp:read",
        ] {
            assert!(require_read_only(scope).is_err());
        }
    }
}
