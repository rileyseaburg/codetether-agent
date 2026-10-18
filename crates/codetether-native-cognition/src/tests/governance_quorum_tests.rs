//! Test for quorum-based proposal verification.

use std::collections::HashMap;

use super::governance_tests_support::{auditor_veto_governance, pending_proposal, tick_once};
use super::tests_request::create;
use super::tests_support::test_runtime;
use super::{ProposalStatus, ProposalVote};

#[tokio::test]
async fn governance_proposal_resolution() {
    let runtime = test_runtime();
    *runtime.governance.write().await = auditor_veto_governance();
    for id in ["voter-1", "voter-2"] {
        runtime
            .create_persona(create(id, "engineer", "vote"))
            .await
            .unwrap();
    }
    let votes = HashMap::from([("voter-1".to_string(), ProposalVote::Approve)]);
    runtime.proposals.write().await.insert(
        "prop-1".to_string(),
        pending_proposal("prop-1", "voter-1", votes),
    );

    // Quorum is 0.5 * 2 = 1, and one approval is present.
    tick_once(&runtime).await;

    let status = runtime.get_proposals().await["prop-1"].status;
    assert!(
        status == ProposalStatus::Verified || status == ProposalStatus::Executed,
        "Expected Verified or Executed, got {status:?}"
    );
}
