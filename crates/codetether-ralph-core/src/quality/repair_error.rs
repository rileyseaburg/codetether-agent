//! Format up to twenty compiler-like error lines for a repair agent.
pub(super) fn format_error(name: &str, output: &str) -> String {
    // Extract error lines for agent feedback
    let errors: String = output
        .lines()
        .filter(|l| l.starts_with("error") || l.contains("error:") || l.contains("error["))
        .take(20)
        .collect::<Vec<_>>()
        .join("\n");
    format!("Quality check `{name}` failed:\n{errors}")
}
