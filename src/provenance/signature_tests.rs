use super::sign_provenance;
use crate::provenance::{ExecutionOrigin, ExecutionProvenance};

#[test]
fn matches_the_server_provenance_hmac_vector() {
    unsafe {
        std::env::set_var("CODETETHER_SIGNING_SECRET", "test-provenance-secret");
    }
    let mut provenance = ExecutionProvenance::for_session("author-session", "author");
    provenance.provenance_id = "ctprov_1234567890abcdef".to_string();
    provenance.task_id = None;
    provenance.run_id = None;
    provenance.attempt_id = None;
    provenance.identity.tenant_id = Some("tenant".to_string());
    provenance.identity.agent_identity_id =
        Some("ctforgejo_9e1fbe45a595b21bd9146db4a011cae0de38f193".to_string());
    provenance.identity.origin = ExecutionOrigin::Worker;
    provenance.identity.worker_id = None;
    provenance.identity.key_id = Some("author-key".to_string());
    provenance.identity.github_installation_id = None;
    provenance.identity.github_app_id = None;
    let signature = sign_provenance(&provenance).unwrap();
    unsafe {
        std::env::remove_var("CODETETHER_SIGNING_SECRET");
    }
    assert_eq!(
        signature,
        "72bc4744ef5ec3461ed0279c9a0d716d12b87d29e711dd8f6263cfe7149005be"
    );
}
