//! fn policy implementation.
use super::*;

impl Dependency {
    /// Stable snake_case encoding — see [`Difficulty::as_str`].
    pub const fn as_str(self) -> &'static str {
        match self {
            Dependency::Isolated => "isolated",
            Dependency::Chained => "chained",
        }
    }
}
