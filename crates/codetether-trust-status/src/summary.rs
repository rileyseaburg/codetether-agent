//! Full and compact trust-policy labels.
use crate::labels;
use codetether_config::TrustPolicyStatus;

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
