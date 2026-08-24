//! Fail-closed validation for sandbox runners without network isolation.

use super::SandboxPolicy;
use anyhow::{Result, bail};

pub(super) fn validate(policy: &SandboxPolicy, network_isolated: bool) -> Result<()> {
    if !policy.allow_network && !network_isolated {
        bail!("sandbox runner cannot enforce disabled network access; refusing execution");
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate;
    use crate::tool::sandbox::SandboxPolicy;

    #[test]
    fn disabled_network_fails_closed_without_isolation() {
        assert!(validate(&SandboxPolicy::default(), false).is_err());
    }

    #[test]
    fn isolation_or_explicit_network_access_allows_execution() {
        assert!(validate(&SandboxPolicy::default(), true).is_ok());
        let policy = SandboxPolicy {
            allow_network: true,
            ..SandboxPolicy::default()
        };
        assert!(validate(&policy, false).is_ok());
    }
}
