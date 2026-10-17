//! detected in shell hook-bypass detection.
use super::*;
// Detection of shell Git invocations that would skip repository hooks.
//
// Commits and pushes must run the repository's pre-commit, post-commit and
// pre-push hooks. `--no-verify` (and commit's `-n`) and hook-path overrides
// bypass them, so they are refused outright.

pub(crate) fn detected(command: &str) -> bool {
    command
        .split([';', '|', '&', '\n', '\r'])
        .any(segment_bypasses_hooks)
}
