use super::context::SignatureContext;

const START: &str = "<!-- codetether-signature:start -->";
const END: &str = "<!-- codetether-signature:end -->";

pub fn merge_signature(base: &str, signature: &SignatureContext) -> String {
    let block = render_signature(signature);
    if let Some((start, end)) = signature_range(base) {
        let mut merged = String::new();
        merged.push_str(base[..start].trim_end());
        if !merged.is_empty() {
            merged.push_str("\n\n");
        }
        merged.push_str(&block);
        let tail = base[end..].trim();
        if !tail.is_empty() {
            merged.push_str("\n\n");
            merged.push_str(tail);
        }
        return merged;
    }
    if base.trim().is_empty() {
        return block;
    }
    format!("{}\n\n{}", base.trim_end(), block)
}

fn render_signature(signature: &SignatureContext) -> String {
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

fn display(value: Option<&str>) -> &str {
    value
        .filter(|value| !value.trim().is_empty())
        .unwrap_or("unknown")
}

fn push_line(lines: &mut Vec<String>, label: &str, value: Option<&str>) {
    if let Some(value) = value.filter(|value| !value.trim().is_empty()) {
        lines.push(format!("- {label}: `{value}`"));
    }
}

fn signature_range(body: &str) -> Option<(usize, usize)> {
    let start = body.find(START)?;
    let end = body[start..].find(END)?;
    Some((start, start + end + END.len()))
}

#[cfg(test)]
mod tests {
    use super::merge_signature;
    use crate::github_pr::context::SignatureContext;

    #[test]
    fn appends_signature_when_missing() {
        let signature = SignatureContext::sample();
        let body = merge_signature("hello", &signature);
        assert!(body.contains("## CodeTether Signature"));
        assert!(body.contains("Model: `zai/glm-5`"));
    }

    #[test]
    fn replaces_existing_signature_block() {
        let signature = SignatureContext::sample();
        let body = merge_signature(
            "hello\n\n<!-- codetether-signature:start -->\nold\n<!-- codetether-signature:end -->",
            &signature,
        );
        assert!(body.contains("Agent ID: `user:test`"));
        assert!(!body.contains("\nold\n"));
    }
}
