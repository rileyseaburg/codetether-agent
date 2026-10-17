//! PR body render signature.
use super::*;
pub(super) fn render_signature(signature: &SignatureContext) -> String {
    let mut lines = vec![
        START.to_string(),
        "## CodeTether Signature".to_string(),
        format!(
            "- Agent ID: `{}`",
            display(signature.agent_identity_id.as_deref())
        ),
        format!("- Model: `{}`", display(signature.model.as_deref())),
        format!("- Commit: `{}`", signature.head_sha),
        format!("- Workspace: `{}`", signature.workspace_id),
    ];
    push_line(&mut lines, "Tenant ID", signature.tenant_id.as_deref());
    push_line(&mut lines, "Worker ID", signature.worker_id.as_deref());
    push_line(&mut lines, "Run ID", signature.run_id.as_deref());
    push_line(&mut lines, "Attempt ID", signature.attempt_id.as_deref());
    push_line(
        &mut lines,
        "GitHub App Installation ID",
        signature.github_installation_id.as_deref(),
    );
    push_line(
        &mut lines,
        "GitHub App ID",
        signature.github_app_id.as_deref(),
    );
    push_line(&mut lines, "Session ID", signature.session_id.as_deref());
    push_line(
        &mut lines,
        "Signature",
        signature.provenance_signature.as_deref(),
    );
    lines.push(END.to_string());
    lines.join("\n")
}
