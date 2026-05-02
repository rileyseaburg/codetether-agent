use super::ValidationReport;

pub(super) fn append_tokens(output: &mut String, report: &ValidationReport) {
    output.push_str(&format!(
        "Token estimate: {} prompt + {} completion = {} total (context: {})\n",
        report.estimated_tokens.prompt_tokens,
        report.estimated_tokens.completion_tokens,
        report.estimated_tokens.total_tokens,
        report.estimated_tokens.context_window
    ));
    if report.estimated_tokens.exceeds_limit {
        output.push_str("  ⚠ Exceeds context window!\n");
    }
}
