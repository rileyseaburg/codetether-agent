//! One complete delivery: plan, persist, apply, record.

use super::{Delivery, DeliveryStatus, apply, connection, ledger, plan};
use anyhow::Result;
use std::path::Path;

pub(super) fn deliver(branch: &str, target: &Path) -> Result<Delivery> {
    let root = plan::root(target)?;
    let db = connection::open(&root)?;
    let plan = plan::compute(&root, branch, ledger::last_delivered(&db, &root, branch)?)?;
    let (id, status, detail) = if plan.base == plan.head {
        // The branch tip was already delivered (or is already in HEAD).
        (0, DeliveryStatus::AlreadyPresent, String::new())
    } else if plan.patch.is_empty() {
        (0, DeliveryStatus::Empty, String::new())
    } else {
        let id = ledger::begin(&db, &plan)?;
        let (status, detail) = apply::apply(&plan.root, &plan.patch)?;
        ledger::finish(&db, id, status, &detail)?;
        (id, status, detail)
    };
    tracing::info!(%branch, target = %plan.root.display(), status = status.as_str(),
        files = plan.files.len(), delivery_id = id, "Delivered worktree branch");
    Ok(Delivery {
        id,
        status,
        branch: plan.branch,
        head: plan.head,
        base: plan.base,
        target: plan.root,
        files: plan.files,
        detail,
    })
}
