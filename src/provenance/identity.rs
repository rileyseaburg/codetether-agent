const GIT_EMAIL_DOMAIN: &str = "codetether.run";
const GIT_NAME: &str = "CodeTether Agent";

pub fn git_author_name(agent_identity_id: Option<&str>) -> String {
    match agent_identity_id.and_then(sanitize_identity) {
        Some(identity) => format!("{GIT_NAME} [{identity}]"),
        None => GIT_NAME.to_string(),
    }
}

pub fn git_author_email(
    agent_identity_id: Option<&str>,
    worker_id: Option<&str>,
    provenance_id: Option<&str>,
) -> String {
    let identity = [agent_identity_id, worker_id, provenance_id]
        .into_iter()
        .flatten()
        .find_map(sanitize_identity)
        .unwrap_or_else(|| "unknown".to_string());
    format!("agent+{identity}@{GIT_EMAIL_DOMAIN}")
}

fn sanitize_identity(value: &str) -> Option<String> {
    let mut output = String::new();
    let mut last_was_dash = false;
    for ch in value.trim().chars() {
        let lower = ch.to_ascii_lowercase();
        if lower.is_ascii_alphanumeric() {
            output.push(lower);
            last_was_dash = false;
        } else if !last_was_dash && !output.is_empty() {
            output.push('-');
            last_was_dash = true;
        }
        if output.len() >= 48 {
            break;
        }
    }
    while output.ends_with('-') {
        output.pop();
    }
    (!output.is_empty()).then_some(output)
}

#[cfg(test)]
mod tests {
    use super::{git_author_email, git_author_name};

    #[test]
    fn derives_unique_email_from_agent_identity() {
        let email = git_author_email(Some("user:abc-123"), None, None);
        assert_eq!(email, "agent+user-abc-123@codetether.run");
    }

    #[test]
    fn includes_identity_in_display_name() {
        let name = git_author_name(Some("user:abc-123"));
        assert_eq!(name, "CodeTether Agent [user-abc-123]");
    }
}
