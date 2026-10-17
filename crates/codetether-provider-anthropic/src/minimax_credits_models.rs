//! MiniMax credits-plan highspeed model metadata.
//!
//! This module lists MiniMax models that are available through the credits-plan
//! highspeed tier. Each entry captures the provider model identifier, display
//! name, context-window limit, maximum output-token limit, tool-support flag,
//! and pricing metadata consumed by the provider model registry.

use super::models::Spec;

/// Model specifications for MiniMax credits-plan highspeed offerings.
///
/// The slice is consumed by the MiniMax model registry when exposing supported
/// models to provider discovery and model-selection code. The order is kept as
/// declared here so newer or preferred highspeed models can be presented before
/// older entries.
///
/// Each [`Spec`] tuple contains, in order:
///
/// * Provider model ID sent to the MiniMax API.
/// * Human-readable display name.
/// * Maximum input context window in tokens.
/// * Maximum output tokens.
/// * Whether the model supports tool use.
/// * Input price metadata.
/// * Output price metadata.
pub(crate) const MODELS: &[Spec] = &[
    Spec(
        "MiniMax-M2.5-highspeed",
        "MiniMax M2.5 Highspeed",
        200_000,
        65_536,
        false,
        0.6,
        2.4,
    ),
    Spec(
        "MiniMax-M2.1-highspeed",
        "MiniMax M2.1 Highspeed",
        200_000,
        65_536,
        false,
        0.6,
        2.4,
    ),
];
