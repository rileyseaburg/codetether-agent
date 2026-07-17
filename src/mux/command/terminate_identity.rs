//! Process identity verification for mux force termination.

use std::ffi::OsString;

/// Confirms a command belongs to the recorded named mux server.
pub(super) fn matches(command: &[OsString], session: &str) -> bool {
    let args: Vec<_> = command
        .iter()
        .map(|value| value.to_string_lossy())
        .collect();
    let serves_mux = args
        .windows(2)
        .any(|pair| pair[0] == "mux" && pair[1] == "serve");
    let owns_session = args
        .windows(2)
        .any(|pair| pair[0] == "--session" && pair[1] == session);
    serves_mux && owns_session
}

#[cfg(test)]
mod tests {
    #[test]
    fn matches_only_the_named_mux_server() {
        let args =
            ["codetether", "mux", "serve", "--session", "work"].map(std::ffi::OsString::from);
        assert!(super::matches(&args, "work"));
        assert!(!super::matches(&args, "other"));
    }
}
