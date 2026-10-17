//! Local session recall evidence with explicitly requested RLM answer synthesis.
//!
//! [SessionRecallTool] validates requests, retrieves through [Host], renders
//! evidence and optionally synthesizes an answer. [RecallHit] and [render]
//! preserve source provenance for other consumers of the local recall index.
mod answer;
mod args;
mod description;
mod execute;
mod faults;
mod hit;
mod host;
pub mod render;
mod rlm_run;
mod schema;
mod tool_impl;
mod tool_struct;
pub use hit::RecallHit;
pub use host::{Host, SearchOptions};
pub use tool_struct::SessionRecallTool;
#[cfg(test)]
mod tests;
