use crate::provenance::{git_author_email, git_author_name};
use serde_json::Value;

pub fn git_identity_env_from_tool_args(args: &Value) -> Vec<(&'static str, String)> {
    let provenance_id = args["__ct_provenance_id"].as_str();
    let agent_identity_id = args["__ct_agent_identity_id"].as_str();
    let worker_id = args["__ct_worker_id"].as_str();
    if provenance_id.is_none() && agent_identity_id.is_none() && worker_id.is_none() {
        return Vec::new();
    }
    let name = git_author_name(agent_identity_id);
    let email = git_author_email(agent_identity_id, worker_id, provenance_id);
    vec![
        ("GIT_AUTHOR_NAME", name.clone()),
        ("GIT_COMMITTER_NAME", name),
        ("GIT_AUTHOR_EMAIL", email.clone()),
        ("GIT_COMMITTER_EMAIL", email),
    ]
}

#[cfg(test)]
mod tests {
    use super::git_identity_env_from_tool_args;
    use serde_json::json;

    #[test]
    fn injects_codetether_git_identity() {
        let env = git_identity_env_from_tool_args(&json!({
            "__ct_provenance_id": "ctprov_123",
            "__ct_agent_identity_id": "user:abc-123",
            "__ct_worker_id": "wrk_1",
        }));
        assert!(
            env.iter()
                .any(|(k, v)| *k == "GIT_AUTHOR_NAME" && v.contains("user-abc-123"))
        );
        assert!(
            env.iter()
                .any(|(k, v)| *k == "GIT_AUTHOR_EMAIL" && v == "agent+user-abc-123@codetether.run")
        );
    }
}
