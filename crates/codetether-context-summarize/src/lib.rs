//! Cached or on-demand context summaries over an existing session store.
//!
//! [ContextSummarizeTool] validates ranges, loads the calling session, returns
//! cached text or invokes the supplied producer and persists its result.
//! [SummarySource] retains application ownership of snapshot and storage details.
mod execute;
mod parse;
mod produce;
mod respond;
mod run;
mod schema;
mod source;
mod tool_struct;
pub use source::SummarySource;
pub use tool_struct::ContextSummarizeTool;
