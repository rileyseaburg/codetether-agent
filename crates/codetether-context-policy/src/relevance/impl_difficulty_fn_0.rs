//! fn policy implementation.
use super::*;

impl Difficulty {
    /// Stable snake_case encoding. Never renamed — used as part of the
    /// persisted [`crate::delegation::DelegationState`] key.
    pub const fn as_str(self) -> &'static str {
        match self {
            Difficulty::Easy => "easy",
            Difficulty::Medium => "medium",
            Difficulty::Hard => "hard",
        }
    }
}
