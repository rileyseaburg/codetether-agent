//! MiniMax coding-plan highspeed model variants.
//!
//! Highspeed variants offer the same quality at higher output throughput
//! (~100 tps vs ~60 tps). Accessible through the Anthropic-compatible endpoint
//! at `https://api.minimax.io/anthropic`.

use super::models::Spec;

pub(crate) const MODELS: &[Spec] = &[
    Spec(
        "MiniMax-M2.7-highspeed",
        "MiniMax M2.7 Highspeed",
        204_800,
        65_536,
        false,
        0.6,
        2.4,
    ),
    Spec(
        "MiniMax-M2.5-highspeed",
        "MiniMax M2.5 Highspeed",
        204_800,
        65_536,
        false,
        0.6,
        2.4,
    ),
    Spec(
        "MiniMax-M2.1-highspeed",
        "MiniMax M2.1 Highspeed",
        204_800,
        65_536,
        false,
        0.6,
        2.4,
    ),
];
