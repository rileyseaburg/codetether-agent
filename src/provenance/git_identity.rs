use super::ExecutionProvenance;
use super::identity::{git_author_email, git_author_name};

pub fn git_identity_env_vars(
    provenance: Option<&ExecutionProvenance>,
) -> Vec<(&'static str, String)> {
    let Some(provenance) = provenance else {
        return Vec::new();
    };
    let name = git_author_name(provenance.identity.agent_identity_id.as_deref());
    let email = git_author_email(
        provenance.identity.agent_identity_id.as_deref(),
        provenance.identity.worker_id.as_deref(),
        Some(provenance.provenance_id.as_str()),
    );
    vec![
        ("GIT_AUTHOR_NAME", name.clone()),
        ("GIT_COMMITTER_NAME", name),
        ("GIT_AUTHOR_EMAIL", email.clone()),
        ("GIT_COMMITTER_EMAIL", email),
    ]
}

#[cfg(test)]
mod tests {
    use super::git_identity_env_vars;
    use crate::provenance::{ExecutionOrigin, ExecutionProvenance};

    #[test]
    fn derives_real_git_identity_env() {
        let mut provenance = ExecutionProvenance::for_operation("worker", ExecutionOrigin::Worker);
        provenance.identity.agent_identity_id = Some("user:abc-123".to_string());
        provenance.identity.worker_id = Some("wrk-1".to_string());
        let env = git_identity_env_vars(Some(&provenance));
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
