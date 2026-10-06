//! Verification audits use the execution identity, not a later settings lookup.

use super::*;
use crate::tool::goal::verify::ReviewExecution;
use async_trait::async_trait;

struct Captured;
#[async_trait]
impl VerifierAgent for Captured {
    async fn review(&self, _: &VerificationRequest) -> anyhow::Result<String> {
        panic!("must use review_with_identity");
    }
    async fn review_with_identity(&self, _: &VerificationRequest) -> ReviewExecution {
        ReviewExecution {
            report: Ok("I am not the routed model.\nVERDICT: PASS".into()),
            identity: "actual-provider/actual-model".into(),
        }
    }
    async fn identity(&self) -> String {
        panic!("must not re-resolve after the run");
    }
}

#[tokio::test]
async fn verifier_identity_gate_uses_only_captured_routing() {
    let request = VerificationRequest {
        objective: "test".into(),
        success_criteria: vec![],
        forbidden: vec![],
        claimed: crate::session::tasks::GoalStatus::Complete,
        evidence: "fixture".into(),
    };
    let (verdict, identity) = verify_with_identity(&Captured, &request).await;
    assert!(matches!(verdict, Verdict::Pass));
    assert_eq!(identity, "actual-provider/actual-model");
}
