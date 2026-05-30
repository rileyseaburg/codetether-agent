//! Edit tool: replace strings in files.

mod args;
mod diff;
mod execute;
mod matcher;
mod metadata;
mod morph;
mod morph_flow;
mod schema;
mod tool_struct;

pub use tool_struct::EditTool;
