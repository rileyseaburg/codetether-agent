//! Restore session grants observed through the durable approval store.

pub(super) fn settle(
    store: &crate::approval::ApprovalStore,
    id: &str,
    decision: &crate::approval::ApprovalDecision,
) -> anyhow::Result<()> {
    let Some(kind) = decision
        .kind
        .filter(|kind| !matches!(kind, crate::approval::ApprovalDecisionKind::ApproveOnce))
    else {
        return Ok(());
    };
    let request = store
        .request(id)?
        .ok_or_else(|| anyhow::anyhow!("approval request not found"))?;
    let receipt = crate::approval::ApprovalReceipt::from_parts(&request, decision);
    kind.grant_session(&receipt);
    Ok(())
}
