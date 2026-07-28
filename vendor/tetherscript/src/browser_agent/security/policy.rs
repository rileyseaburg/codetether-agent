//! Security policy metadata for origin decisions.

use super::{Origin, SandboxFlags};

/// Page or context policy for origin decisions.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::{Origin, SecurityPolicy};
///
/// let mut policy = SecurityPolicy::default();
/// policy.allow_origin(Origin::parse("https://api.test"));
/// assert!(policy.allows(&Origin::parse("https://app.test"), &Origin::parse("https://api.test")));
/// ```
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub struct SecurityPolicy {
    allowed_origins: Vec<Origin>,
    /// Sandbox metadata associated with this policy.
    pub sandbox: SandboxFlags,
}

impl SecurityPolicy {
    /// Return true when target is same-origin or explicitly allowed.
    pub fn allows(&self, source: &Origin, target: &Origin) -> bool {
        source.is_same_origin(target) || self.allowed_origins.iter().any(|item| item == target)
    }

    /// Add an explicitly allowed cross-origin target.
    pub fn allow_origin(&mut self, origin: Origin) {
        if !self.allowed_origins.contains(&origin) {
            self.allowed_origins.push(origin);
        }
    }

    /// Remove all explicitly allowed cross-origin targets.
    pub fn clear_allowed_origins(&mut self) {
        self.allowed_origins.clear();
    }

    /// Return explicitly allowed cross-origin targets.
    pub fn allowed_origins(&self) -> &[Origin] {
        &self.allowed_origins
    }
}
