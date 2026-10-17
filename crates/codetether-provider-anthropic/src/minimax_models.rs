//! MiniMax coding-plan model metadata.

use super::models::Spec;

pub(crate) const MODELS: &[Spec] = &[
    Spec(
        "MiniMax-M3",
        "MiniMax M3",
        1_000_000,
        128_000,
        true,
        0.3,
        1.2,
    ),
    Spec(
        "MiniMax-M2.5",
        "MiniMax M2.5",
        200_000,
        65_536,
        false,
        0.3,
        1.2,
    ),
    Spec(
        "MiniMax-M2.1",
        "MiniMax M2.1",
        200_000,
        65_536,
        false,
        0.3,
        1.2,
    ),
    Spec("MiniMax-M2", "MiniMax M2", 200_000, 65_536, false, 0.3, 1.2),
];
