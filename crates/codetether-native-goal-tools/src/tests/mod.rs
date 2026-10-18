//! Native goal-edit and audited-transition regressions.
mod edit_tests;
mod update_run;
use codetether_goal_verification::{VerificationRequest, VerifierAgent};
use std::path::PathBuf;
type TestHost = crate::Host<InjectedOnly>;
struct InjectedOnly;
#[async_trait::async_trait]
impl VerifierAgent for InjectedOnly {
    async fn review(&self, _: &VerificationRequest) -> anyhow::Result<String> {
        panic!("goal tests must inject their scripted verifier");
    }
}
impl crate::VerifierFactory for InjectedOnly {
    fn create(_: Option<String>, _: PathBuf) -> Self {
        panic!("goal tests must not construct a provider-backed verifier");
    }
}
mod edit {
    pub(super) type EditGoalTool = codetether_goal_tools::EditGoalTool<super::TestHost>;
    #[allow(non_upper_case_globals)]
    pub(super) const EditGoalTool: EditGoalTool = EditGoalTool::new();
}
