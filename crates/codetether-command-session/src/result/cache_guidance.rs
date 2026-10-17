//! Safe recovery hints for read-only Cargo caches in guarded commands.

pub(super) fn annotate(output: String, sandboxed: bool) -> String {
    if !sandboxed
        || !output.contains("failed to write cache")
        || !(output.contains("Permission denied") || output.contains("Read-only file system"))
    {
        return output;
    }
    format!(
        "{output}\n\nSandbox recovery: approving a command authorizes the invocation, not additional writable paths. \
        Put mutable Cargo state under the workspace, for example CARGO_HOME=\"$PWD/.codetether/cache/cargo\" \
        (keep RUSTUP_HOME on the existing toolchain). Downloads still require network permission. \
        Retrying the same approval cannot change cache permissions; do not disable the sandbox or chmod/chown the host cache. \
        This cache warning alone does not establish that the command failed; check its exit status."
    )
}

#[cfg(test)]
mod tests {
    use super::annotate;

    #[test]
    fn sandboxed_cache_warning_has_safe_recovery_and_preserves_diagnostic() {
        let raw = "warning: failed to write cache, error: Permission denied (os error 13)";
        let text = annotate(raw.into(), true);
        assert!(text.starts_with(raw));
        assert!(text.contains("CARGO_HOME=\"$PWD/.codetether/cache/cargo\""));
        assert!(text.contains("not additional writable paths"));
        assert!(text.contains("check its exit status"));
    }

    #[test]
    fn unrelated_and_unsandboxed_failures_are_not_misdiagnosed() {
        let raw = "failed to write cache: Permission denied";
        assert_eq!(annotate(raw.into(), false), raw);
        assert_eq!(
            annotate("Permission denied".into(), true),
            "Permission denied"
        );
        assert_eq!(annotate("all good".into(), true), "all good");
    }
}
