//! KrOutcomeType serialized state.

use serde::{Deserialize, Serialize};

/// Type of key result outcome
///
/// # Examples
/// ```
/// use codetether_okr::{KrOutcomeType};
/// let value = KrOutcomeType::Evidence;
/// assert!(matches!(value, KrOutcomeType::Evidence));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[derive(Default)]
pub enum KrOutcomeType {
    /// Evidence/documentation
    #[default]
    Evidence,

    /// Test pass
    TestPass,

    /// Code change
    CodeChange,

    /// Bug fix
    BugFix,

    /// Feature delivered
    FeatureDelivered,

    /// Metric achievement
    MetricAchievement,

    /// Review passed
    ReviewPassed,

    /// Deployment
    Deployment,
}
