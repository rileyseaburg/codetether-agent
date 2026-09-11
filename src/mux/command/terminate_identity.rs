//! Process identity verification for mux force termination.

use std::ffi::OsString;
use std::path::Path;

/// Confirms a command is the mux server bound to `workspace`.
///
/// The `--session` flag only names the first session a server hosted, so the
/// workspace directory is the stable identity for the process's lifetime.
pub(super) fn matches(command: &[OsString], workspace: &Path) -> bool {
    let args: Vec<_> = command
        .iter()
        .map(|value| value.to_string_lossy())
        .collect();
    let serves_mux = args
        .windows(2)
        .any(|pair| pair[0] == "mux" && pair[1] == "serve");
    let owns_workspace = args
        .windows(2)
        .any(|pair| pair[0] == "--directory" && Path::new(pair[1].as_ref()) == workspace);
    serves_mux && owns_workspace
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    #[test]
    fn matches_only_the_server_for_the_workspace() {
        let args = [
            "codetether",
            "mux",
            "serve",
            "--session",
            "work",
            "--directory",
            "/repo",
        ]
        .map(std::ffi::OsString::from);
        assert!(super::matches(&args, Path::new("/repo")));
        assert!(!super::matches(&args, Path::new("/other")));
    }
}
