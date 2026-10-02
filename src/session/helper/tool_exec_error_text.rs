//! Preserve the entire anyhow cause chain when presenting a tool failure.

pub(super) fn diagnostic(error: &anyhow::Error) -> String {
    format!("{error:#}")
}

#[cfg(test)]
mod tests {
    use super::diagnostic;

    #[test]
    fn tool_failure_keeps_permission_denial_under_context() {
        let error = anyhow::Error::new(std::io::Error::from_raw_os_error(13))
            .context("starting sandbox")
            .context("approved exec_command failed");
        let text = diagnostic(&error);
        assert!(text.starts_with("approved exec_command failed: starting sandbox:"));
        assert!(text.contains("os error 13"));
    }

    #[test]
    fn tool_failure_without_context_is_unchanged() {
        let error = anyhow::anyhow!("worker stopped");
        assert_eq!(diagnostic(&error), "worker stopped");
    }
}
