use super::labels;
use crate::config::{ApprovalPolicy, TrustPolicyStatus};

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
        return format!("Policy status unavailable. {}", change_hint());
    };
    format!(
        "Policy: {}; {}. {}",
        summary(&status),
        behavior(status.approval_policy),
        change_hint()
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

fn change_hint() -> &'static str {
    "Use `/access-mode approve` for fewer prompts or `/access-mode full` for none"
}
