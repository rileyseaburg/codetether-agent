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
