//! BrowserContext security metadata APIs.

use super::{Origin, SecurityPolicy};
use crate::browser_agent::context::BrowserContext;

impl BrowserContext {
    /// Return the context default security policy.
    pub fn security_policy(&self) -> &SecurityPolicy {
        &self.security_policy
    }

    /// Replace the context policy and apply it to existing pages.
    pub fn set_security_policy(&mut self, policy: SecurityPolicy) {
        self.security_policy = policy;
        self.apply_security_policy();
    }

    /// Allow a cross-origin target for all pages in this context.
    pub fn allow_origin(&mut self, url: impl AsRef<str>) -> Origin {
        let origin = Origin::parse(url.as_ref());
        self.security_policy.allow_origin(origin.clone());
        self.apply_security_policy();
        origin
    }

    /// Clear context cross-origin allowances and apply to existing pages.
    pub fn clear_allowed_origins(&mut self) {
        self.security_policy.clear_allowed_origins();
        self.apply_security_policy();
    }

    fn apply_security_policy(&mut self) {
        for page in &mut self.pages {
            page.set_security_policy(self.security_policy.clone());
        }
    }
}
