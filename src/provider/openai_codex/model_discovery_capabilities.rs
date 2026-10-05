//! Capability entries decoded from the authenticated Codex catalog.
use serde::Deserialize;
/// One advertised reasoning effort.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub(crate) struct ReasoningEffort {
    /// Wire effort, such as high or ultra.
    pub effort: String,
}
/// One advertised service tier.
#[derive(Clone, Debug, Deserialize, PartialEq, Eq)]
pub(crate) struct ServiceTier {
    /// Wire tier, such as priority or ultrafast.
    pub id: String,
}
