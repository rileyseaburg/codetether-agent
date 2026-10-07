use super::types::AgentIdentity;
use super::{ClaimProvenance, ExecutionOrigin, ExecutionProvenance};

fn provenance(identity: Option<&str>) -> ExecutionProvenance {
    ExecutionProvenance {
        provenance_id: "provenance-1".to_string(),
        session_id: None,
        task_id: None,
        run_id: None,
        attempt_id: None,
        issued_at: chrono::Utc::now(),
        identity: AgentIdentity {
            tenant_id: None,
            agent_identity_id: identity.map(ToString::to_string),
            agent_name: "author-agent".to_string(),
            origin: ExecutionOrigin::LocalCli,
            worker_id: None,
            key_id: None,
            github_installation_id: None,
            github_app_id: None,
        },
    }
}

fn claim(identity: &str) -> ClaimProvenance {
    ClaimProvenance {
        worker_id: "worker-1".to_string(),
        task_id: "task-1".to_string(),
        agent_identity_id: Some(identity.to_string()),
        ..ClaimProvenance::default()
    }
}

#[test]
fn claim_preserves_durable_runtime_identity() {
    let mut provenance = provenance(Some("durable-author-identity"));
    provenance.apply_claim(&claim("user:reviewer"));

    assert_eq!(
        provenance.identity.agent_identity_id.as_deref(),
        Some("durable-author-identity")
    );
}

#[test]
fn claim_fills_missing_runtime_identity() {
    let mut provenance = provenance(None);
    provenance.apply_claim(&claim("server-agent-identity"));
    assert_eq!(
        provenance.identity.agent_identity_id.as_deref(),
        Some("server-agent-identity")
    );
}
