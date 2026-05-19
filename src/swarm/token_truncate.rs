//! Token-truncation helpers for sub-agent context management.

#[path = "token_limits/constants.rs"]
mod constants;
#[path = "token_limits/estimation.rs"]
mod estimation;
#[path = "token_limits/message_truncation.rs"]
mod message_truncation;
#[path = "token_limits/summary.rs"]
mod summary;
#[path = "token_limits/truncation.rs"]
mod truncation;

pub use constants::{DEFAULT_CONTEXT_LIMIT, RESPONSE_RESERVE_TOKENS, TRUNCATION_THRESHOLD};
pub use estimation::{estimate_message_tokens, estimate_tokens, estimate_total_tokens};
pub use message_truncation::truncate_messages_to_fit;
pub use summary::summarize_removed_messages;
pub use truncation::{truncate_large_tool_results, truncate_single_result};
