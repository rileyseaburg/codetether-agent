//! Worker task-claim response lifecycle.
//!
//! [claim_task] requests a claim and updates progress before returning
//! [ClaimedTaskData]. Rejections retain the server status and bounded body text.
use super::summarize_response_body;
use anyhow::Result;
use codetether_a2a_worker_core::{claim::TaskClaimResponse, task_timeline};
mod record;
mod rejected;
mod request;
pub use record::ClaimedTaskData;
pub use request::claim_task;
