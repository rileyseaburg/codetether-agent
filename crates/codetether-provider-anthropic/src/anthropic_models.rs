//! Anthropic Claude model metadata.

use super::models::Spec;

pub(crate) const MODELS: &[Spec] = &[
    Spec(
        "claude-sonnet-4-6",
        "Claude Sonnet 4.6",
        200_000,
        128_000,
        true,
        3.0,
        15.0,
    ),
    Spec(
        "claude-sonnet-4-20250514",
        "Claude Sonnet 4",
        200_000,
        64_000,
        true,
        3.0,
        15.0,
    ),
    Spec(
        "claude-opus-4-20250514",
        "Claude Opus 4",
        200_000,
        32_000,
        true,
        15.0,
        75.0,
    ),
    Spec(
        "claude-haiku-3-5-20241022",
        "Claude 3.5 Haiku",
        200_000,
        8_192,
        true,
        0.80,
        4.0,
    ),
];
