//! hook skip env in shell hook-bypass detection.
use super::*;

/// Environment assignments that make repository hook managers skip checks.
pub(super) fn hook_skip_env(word: &str) -> bool {
    let Some((name, _)) = word.split_once('=') else {
        return false;
    };
    matches!(
        name,
        "SKIP" | "PRE_COMMIT_ALLOW_NO_CONFIG" | "HUSKY" | "LEFTHOOK"
    ) || name.starts_with("GIT_CONFIG_KEY_")
        || name == "GIT_CONFIG_PARAMETERS"
}
