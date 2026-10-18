//! Approval behavior explanations and startup guidance.
pub(super) use crate::summary::{badge, summary};
use codetether_config::{ApprovalPolicy, TrustPolicyStatus};
const CHANGE_HINT: &str = "Settings controls Access mode and Network access separately; approval does not override sandbox limits";

pub(super) fn approval(status: Option<TrustPolicyStatus>) -> String {
    let Some(status) = status else {
        return format!("Policy status unavailable. {CHANGE_HINT}");
    };
    format!(
        "Policy: {}; {}. {}",
        summary(&status),
        behavior(status.approval_policy),
        CHANGE_HINT
    )
}

pub(super) fn startup(status: TrustPolicyStatus) -> String {
    format!("{}; {}", summary(&status), behavior(status.approval_policy))
}

fn behavior(policy: ApprovalPolicy) -> &'static str {
    match policy {
        ApprovalPolicy::Untrusted | ApprovalPolicy::OnRequest => "mutating tools require approval",
        ApprovalPolicy::OnFailure => "tools run unless escalation is needed",
        ApprovalPolicy::Never => "approval prompts are disabled",
    }
}
