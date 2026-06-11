//! Process-local approval grants for the current interactive session.

use std::collections::HashSet;
use std::sync::{Mutex, OnceLock};

use super::ApprovalReceipt;

type Grant = (String, String, String);

static GRANTS: OnceLock<Mutex<HashSet<Grant>>> = OnceLock::new();

fn grants() -> &'static Mutex<HashSet<Grant>> {
    GRANTS.get_or_init(|| Mutex::new(HashSet::new()))
}

pub fn grant(receipt: &ApprovalReceipt) {
    grants()
        .lock()
        .expect("session approval grants lock")
        .insert(tuple(&receipt.tool, &receipt.action, &receipt.resource));
}

pub fn allowed(tool: &str, action: &str, resource: &str) -> bool {
    grants()
        .lock()
        .expect("session approval grants lock")
        .contains(&tuple(tool, action, resource))
}

fn tuple(tool: &str, action: &str, resource: &str) -> Grant {
    (tool.to_string(), action.to_string(), resource.to_string())
}

#[cfg(test)]
pub(crate) fn reset() {
    grants()
        .lock()
        .expect("session approval grants lock")
        .clear();
}
