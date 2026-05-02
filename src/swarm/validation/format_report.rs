use super::ValidationReport;

impl ValidationReport {
    /// Format the report as a human-readable string.
    pub fn format(&self) -> String {
        let mut output = String::new();
        append_summary(&mut output, self.is_valid);
        append_provider(&mut output, self);
        super::format_workspace::append_workspace(&mut output, self);
        super::format_tokens::append_tokens(&mut output, self);
        super::format_issue::append_issues(&mut output, &self.issues);
        output
    }
}

fn append_summary(output: &mut String, is_valid: bool) {
    if is_valid {
        output.push_str("✓ Swarm pre-flight validation passed\n\n");
    } else {
        output.push_str("✗ Swarm pre-flight validation failed\n\n");
    }
}

fn append_provider(output: &mut String, report: &ValidationReport) {
    let status = if report.provider_status.is_available {
        "✓ Available"
    } else {
        "✗ Unavailable"
    };
    output.push_str(&format!(
        "Provider: {} ({}) - {}\n",
        report.provider_status.provider, report.provider_status.model, status
    ));
}
