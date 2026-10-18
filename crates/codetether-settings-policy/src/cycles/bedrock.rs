//! Bedrock tier and effort update sequencing.
use super::{Cycle, Runtime};
pub(super) fn tier<R: Runtime>() -> String {
    let current = crate::bedrock_service_tier_label(
        R::current(Cycle::BedrockTier).unwrap_or_default().as_str(),
    );
    let next = crate::next_bedrock_service_tier(current);
    R::set(Cycle::BedrockTier, next);
    format!(
        "Bedrock service tier: {}",
        crate::bedrock_service_tier_label(
            R::current(Cycle::BedrockTier).unwrap_or_default().as_str()
        )
    )
}
pub(super) fn effort<R: Runtime>() -> String {
    let current = crate::bedrock_thinking_effort_label(
        R::current(Cycle::BedrockEffort)
            .unwrap_or_default()
            .as_str(),
    );
    let next = crate::next_bedrock_thinking_effort(current);
    R::set(Cycle::BedrockEffort, Some(next.to_string()));
    format!("Bedrock thinking effort: {next}")
}
