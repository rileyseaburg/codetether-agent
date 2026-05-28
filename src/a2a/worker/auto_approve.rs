//! Auto-approval policy parsing for worker tool execution.

/// Approval policy used by the worker when deciding whether to execute tools automatically.
#[derive(Debug, Clone, Copy)]
pub enum AutoApprove {
    All,
    Safe,
    None,
}

pub(super) fn parse_auto_approve(value: &str) -> AutoApprove {
    match value {
        "all" => AutoApprove::All,
        "safe" => AutoApprove::Safe,
        _ => AutoApprove::None,
    }
}
