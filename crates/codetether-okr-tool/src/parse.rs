//! parse for OKR tool dispatch.
use super::*;

pub(super) fn parse_uuid(s: &str, field: &str) -> Result<Uuid> {
    Uuid::parse_str(s).with_context(|| format!("Invalid UUID for {field}: {s}"))
}

pub(super) fn parse_okr_status(s: &str) -> Result<OkrStatus> {
    match s {
        "draft" => Ok(OkrStatus::Draft),
        "active" => Ok(OkrStatus::Active),
        "completed" => Ok(OkrStatus::Completed),
        "cancelled" => Ok(OkrStatus::Cancelled),
        "on_hold" => Ok(OkrStatus::OnHold),
        _ => anyhow::bail!(
            "Unknown OKR status: {s}. Use: draft, active, completed, cancelled, on_hold"
        ),
    }
}

pub(super) fn parse_run_status(s: &str) -> Result<OkrRunStatus> {
    match s {
        "draft" => Ok(OkrRunStatus::Draft),
        "pending_approval" => Ok(OkrRunStatus::PendingApproval),
        "approved" => Ok(OkrRunStatus::Approved),
        "running" => Ok(OkrRunStatus::Running),
        "paused" => Ok(OkrRunStatus::Paused),
        "waiting_approval" => Ok(OkrRunStatus::WaitingApproval),
        "completed" => Ok(OkrRunStatus::Completed),
        "failed" => Ok(OkrRunStatus::Failed),
        "denied" => Ok(OkrRunStatus::Denied),
        "cancelled" => Ok(OkrRunStatus::Cancelled),
        _ => anyhow::bail!("Unknown run status: {s}"),
    }
}
