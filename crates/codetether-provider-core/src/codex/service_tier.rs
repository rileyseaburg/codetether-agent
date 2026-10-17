//! Service tiers understood by Codex request aliases.

/// Service tier selected by a supported Codex model alias.
///
/// # Examples
/// ```
/// use codetether_provider_core::codex::CodexServiceTier;
/// assert_eq!(CodexServiceTier::Priority.as_str(), "priority");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CodexServiceTier {
    Priority,
    Ultrafast,
}

impl CodexServiceTier {
    /// Return the service-tier value sent on the wire.
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Priority => "priority",
            Self::Ultrafast => "ultrafast",
        }
    }
}
