//! key policy implementation.
use super::*;

impl DelegationState {
    /// Encode `(agent, skill, bucket)` as a flat string key.
    pub fn key(agent: &str, skill: &str, bucket: Bucket) -> String {
        format!(
            "{agent}|{skill}|{}|{}|{}",
            bucket.difficulty.as_str(),
            bucket.dependency.as_str(),
            bucket.tool_use.as_str(),
        )
    }
}
