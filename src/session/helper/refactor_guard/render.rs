use super::types::GuardReport;

pub fn prompt(report: &GuardReport) -> String {
    let mut out = String::from(
        "Refactor guard rejected this change. Do not finish yet. Continue the work and satisfy the configured encapsulation and line-budget rules.\n\n",
    );
    for violation in &report.violations {
        out.push_str("- ");
        out.push_str(&violation.path);
        out.push_str(": ");
        out.push_str(&violation.message);
        out.push('\n');
    }
    out.trim_end().to_string()
}
