//! Approve or deny through the shared store and print the decision receipt.

use super::args::ApprovalDecisionArgs;
use super::defaults;
use anyhow::Result;
use codetether_approval_core::ApprovalStore;

pub(crate) fn approve(store: &ApprovalStore, args: ApprovalDecisionArgs) -> Result<()> {
    let actor = defaults::actor(args.actor);
    let reason = defaults::reason(args.reason, "approved");
    let receipt = store.approve(&args.id, &actor, &reason)?;
    println!(
        "approved {} via {}",
        receipt.approval_id, receipt.decision_id
    );
    Ok(())
}

pub(crate) fn deny(store: &ApprovalStore, args: ApprovalDecisionArgs) -> Result<()> {
    let actor = defaults::actor(args.actor);
    let reason = defaults::reason(args.reason, "denied");
    let decision = store.deny(&args.id, &actor, &reason)?;
    println!("denied {} via {}", args.id, decision.id);
    Ok(())
}
