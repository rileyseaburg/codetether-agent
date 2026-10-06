//! Acquired mutation claim and provenance exposed to resource writers.

use std::collections::BTreeMap;

use super::guard::LeaseGuard;

pub struct MutationClaim {
    guard: LeaseGuard,
    holder: String,
    operation: String,
    field_manager: String,
}

impl MutationClaim {
    pub(super) fn new(
        guard: LeaseGuard,
        holder: String,
        operation: String,
        field_manager: String,
    ) -> Self {
        Self {
            guard,
            holder,
            operation,
            field_manager,
        }
    }

    pub fn field_manager(&self) -> &str {
        &self.field_manager
    }

    pub fn annotations(&self) -> BTreeMap<String, String> {
        [
            ("codetether.run/lease-holder".into(), self.holder.clone()),
            ("codetether.run/operation".into(), self.operation.clone()),
        ]
        .into()
    }

    pub async fn release(self) {
        self.guard.release().await;
    }
}
