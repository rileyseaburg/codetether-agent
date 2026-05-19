//! Structured token-exhaustion reporting for sub-agents.

#[path = "token_limits/evidence.rs"]
mod evidence;
#[path = "token_limits/exhaustion_report.rs"]
mod exhaustion_report;
#[path = "token_limits/recovery_advice.rs"]
mod recovery_advice;

pub use evidence::extract_evidence;
pub use exhaustion_report::TokenExhaustionReport;
pub use recovery_advice::RecoveryAdvice;
