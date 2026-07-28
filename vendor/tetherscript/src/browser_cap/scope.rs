//! Scope, origin, and action policy checks for browser capabilities.

use super::authority::BrowserAuthority;

pub(crate) fn all_scopes() -> Vec<String> {
    [
        "browser.navigate",
        "browser.interact",
        "browser.inspect.dom",
        "browser.inspect.network",
        "browser.inspect.console",
        "browser.inspect.storage",
        "browser.inspect.react",
        "browser.mutate.storage",
        "browser.replay.network",
        "browser.screenshot",
        "browser.visual",
    ]
    .iter()
    .map(|s| s.to_string())
    .collect()
}

impl BrowserAuthority {
    pub(crate) fn require_scope(&self, scope: &str) -> Result<(), String> {
        self.allowed_scopes
            .contains(scope)
            .then_some(())
            .ok_or_else(|| format!("browser: scope `{}` not granted", scope))
    }

    pub(crate) fn require_origin_url(&self, url: &str) -> Result<(), String> {
        if self.allowed_origins.is_empty() || !url.contains("://") {
            return Ok(());
        }
        let parsed = super::origin::ParsedUrl::parse(url)?;
        let origin = parsed.origin();
        if !self.allowed_origins.iter().any(|o| o == &origin) {
            return Err(format!("browser: origin {} is not granted", origin));
        }
        match &self.path_prefix {
            Some(prefix) if !parsed.path.starts_with(prefix) => Err(format!(
                "browser: path {} must start with {}",
                parsed.path, prefix
            )),
            _ => Ok(()),
        }
    }
}
