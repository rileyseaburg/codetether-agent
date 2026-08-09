//! Shared test fixtures for the `rg` tool suites.

use crate::tool::ripgrep::args::RgArgs;

/// Build default [`RgArgs`] carrying only `pattern`.
pub(super) fn args(pattern: &str) -> RgArgs {
    RgArgs {
        pattern: pattern.into(),
        ..Default::default()
    }
}
