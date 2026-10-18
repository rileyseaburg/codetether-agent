//! Project the event ledger into ordered requests and decisions.

use anyhow::Result;
use codetether_approval_core::{
    ApprovalDecision, ApprovalEvent, ApprovalRequest, ApprovalStatus, ApprovalStore,
};

#[derive(Debug, Clone)]
pub(crate) struct ApprovalRecord {
    pub request: ApprovalRequest,
    pub decision: Option<ApprovalDecision>,
}

impl ApprovalRecord {
    pub fn status(&self) -> ApprovalStatus {
        self.decision
            .as_ref()
            .map(|decision| decision.status)
            .unwrap_or(ApprovalStatus::Pending)
    }
}

pub(crate) fn load(store: &ApprovalStore) -> Result<Vec<ApprovalRecord>> {
    let mut records = Vec::new();
    for event in store.events()? {
        match event {
            ApprovalEvent::Request { request } => records.push(ApprovalRecord {
                request,
                decision: None,
            }),
            ApprovalEvent::Decision { decision } => apply_decision(&mut records, decision),
        }
    }
    records.sort_by(|a, b| a.request.requested_at.cmp(&b.request.requested_at));
    Ok(records)
}

fn apply_decision(records: &mut [ApprovalRecord], decision: ApprovalDecision) {
    if let Some(record) = records
        .iter_mut()
        .find(|record| record.request.id == decision.request_id)
    {
        record.decision = Some(decision);
    }
}
