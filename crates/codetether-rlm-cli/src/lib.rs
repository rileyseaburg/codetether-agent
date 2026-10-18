//! RLM command input and report presentation independent of execution.
//!
//! Input selection retains file/stdin behavior; report helpers preserve the
//! text and JSON oracle result contracts used by the command-line application.
mod content_type;
mod input;
mod report;
mod report_base;
mod status;
pub use content_type::detect_rlm_content_type;
pub use input::gather_rlm_content;
pub use report::oracle_json_value;
pub use status::oracle_status_line;

mod analysis;
mod execute;
mod oracle;
mod oracle_persist;
mod oracle_split;
mod oracle_stats;
mod oracle_validate;
mod output;
mod output_json;
mod output_text;
mod provider;
mod validation;
pub use execute::execute;
pub use provider::ProviderSource;
type RunRecord = (
    codetether_rlm::RlmAnalysisResult,
    Vec<codetether_rlm::TraceStep>,
    codetether_rlm::context_trace::ContextTraceSummary,
);
