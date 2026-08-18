//! Test for veto-based proposal rejection.

use std::collections::HashMap;

use super::governance_tests_support::{auditor_veto_governance, pending_proposal, tick_once};
use super::tests_request::create;
use super::tests_support::test_runtime;
use super::{ProposalStatus, ProposalVote};

#[tokio::test]
async fn veto_rejects_proposal() {
    let runtime = test_runtime();
    *runtime.governance.write().await = auditor_veto_governance();
    runtime
        .create_persona(create("eng", "engineer", "build"))
        .await
        .unwrap();
    runtime
        .create_persona(create("aud", "auditor", "audit"))
        .await
        .unwrap();

    let votes = HashMap::from([
        ("eng".to_string(), ProposalVote::Approve),
        ("aud".to_string(), ProposalVote::Veto),
    ]);
    runtime.proposals.write().await.insert(
        "prop-veto".to_string(),
        pending_proposal("prop-veto", "eng", votes),
    );
    tick_once(&runtime).await;

    let proposals = runtime.get_proposals().await;
    assert_eq!(proposals["prop-veto"].status, ProposalStatus::Rejected);
}
