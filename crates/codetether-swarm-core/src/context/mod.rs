//! Context budgeting, history truncation and token-exhaustion recovery.
//!
//! Estimate [codetether_provider_core::Message] histories, truncate tool output
//! or older messages, and capture [TokenExhaustionReport] and [RecoveryAdvice].
//! All operations act on supplied data; this module performs no provider calls.
mod constants;
mod estimation;
mod evidence;
mod exhaustion_report;
mod message_truncation;
mod recovery_advice;
mod summary;
mod truncation;
pub use constants::{DEFAULT_CONTEXT_LIMIT, RESPONSE_RESERVE_TOKENS, TRUNCATION_THRESHOLD};
pub use estimation::{estimate_message_tokens, estimate_tokens, estimate_total_tokens};
pub use evidence::extract_evidence;
pub use exhaustion_report::TokenExhaustionReport;
pub use message_truncation::truncate_messages_to_fit;
pub use recovery_advice::RecoveryAdvice;
pub use summary::summarize_removed_messages;
pub use truncation::{truncate_large_tool_results, truncate_single_result};

mod dependencies;
pub use dependencies::dependency_context;
