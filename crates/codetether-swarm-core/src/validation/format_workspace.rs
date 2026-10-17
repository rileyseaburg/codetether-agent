use super::ValidationReport;

pub(super) fn append_workspace(output: &mut String, report: &ValidationReport) {
    output.push_str(&format!("Workspace: {}\n", workspace_label(report)));
    if report.workspace_status.uncommitted_changes > 0 {
        output.push_str(&format!(
            "  ⚠ {} uncommitted change(s)\n",
            report.workspace_status.uncommitted_changes
        ));
    }
}

fn workspace_label(report: &ValidationReport) -> String {
    if report.workspace_status.is_git_repo {
        format!(
            "✓ Git repo (branch: {})",
            report
                .workspace_status
                .current_branch
                .as_deref()
                .unwrap_or("unknown")
        )
    } else {
        "✗ Not a git repo".to_string()
    }
}
