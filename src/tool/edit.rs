//! Edit tool: replace strings in files.

mod args;
mod diff;
mod execute;
mod matcher;
mod metadata;
mod morph;
mod morph_flow;
pub(crate) mod proposed;
mod schema;
mod tool_struct;

pub use tool_struct::EditTool;
