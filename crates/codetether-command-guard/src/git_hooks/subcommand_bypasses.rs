//! subcommand bypasses in shell hook-bypass detection.
use super::*;

pub(super) fn subcommand_bypasses(sub: &str, rest: &[&str]) -> bool {
    match sub {
        "commit" => rest.iter().any(|arg| *arg == "--no-verify" || short_n(arg)),
        "merge" | "push" | "cherry-pick" | "revert" | "am" | "rebase" => {
            rest.iter().any(|arg| *arg == "--no-verify")
        }
        "config" => rest.iter().any(|arg| hooks_config(arg)),
        _ => false,
    }
}
