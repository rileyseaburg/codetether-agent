//! TetherScript-backed plugin tool.

pub mod convert;
mod errors;
mod execute;
mod input;
mod join;
mod load;
mod partner;
mod result;
mod runner;
mod schema;
mod task;
mod tool;

#[cfg(test)]
mod tests;

pub use tool::TetherScriptPluginTool;

pub(crate) use partner::register;
