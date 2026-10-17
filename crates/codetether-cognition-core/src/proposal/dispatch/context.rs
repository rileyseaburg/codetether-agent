//! Shared stores required by the verified-proposal sweep.
use crate::{Proposal, execution::DecisionReceipt};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Shared handles the execution sweep needs.
///
/// # Examples
/// ```no_run
/// # use std::{collections::HashMap, sync::Arc};
/// # use tokio::sync::RwLock;
/// # use codetether_cognition_core::{Proposal, execution::DecisionReceipt, proposal::dispatch::ExecuteCtx};
/// let proposals = Arc::new(RwLock::new(HashMap::new()));
/// let receipts = Arc::new(RwLock::new(Vec::new()));
/// let pending_approvals = Arc::new(RwLock::new(HashMap::new()));
/// let _ctx = ExecuteCtx { proposals: &proposals, receipts: &receipts, pending_approvals: &pending_approvals };
/// ```
pub struct ExecuteCtx<'a> {
    /// Proposal records available for the sweep.
    pub proposals: &'a Arc<RwLock<HashMap<String, Proposal>>>,
    /// Destination for successful decision receipts.
    pub receipts: &'a Arc<RwLock<Vec<DecisionReceipt>>>,
    /// Human-approval decisions by proposal ID.
    pub pending_approvals: &'a Arc<RwLock<HashMap<String, bool>>>,
}
