use super::labels;
use crate::config::{ApprovalPolicy, TrustPolicyStatus};

const CHANGE_HINT: &str = "Settings controls Access mode and Network access separately; approval does not override sandbox limits";

pub(super) fn summary(status: &TrustPolicyStatus) -> String {
    format!(
        "Access: {} | Trust: {} | Approval: {} | Sandbox: {} | Profile: {}",
        labels::access(status.access_mode),
        labels::trust(status),
        labels::approval(status.approval_policy),
        labels::sandbox(status.sandbox_mode),
        labels::profile(status.permission_profile),
    )
}

pub(super) fn badge(status: &TrustPolicyStatus) -> String {
    format!(
        " ACCESS {} | APPROVAL {} | SANDBOX {} | TRUST {} ",
        labels::access(status.access_mode),
        labels::approval(status.approval_policy),
        labels::sandbox(status.sandbox_mode),
        labels::trust(status),
    )
}

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
