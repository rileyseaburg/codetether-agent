//! Execute TetherScript plugin hooks from inline source or workspace files.
//!
//! [TetherScriptPluginTool] loads and validates source, prepares a bounded task,
//! and converts the result to the shared tool contract. [Host] supplies the
//! application's computer-use backend. [input] describes accepted arguments.
mod errors;
mod execute;
pub mod input;
mod join;
mod load;
mod result;
mod schema;
mod task;
mod tool;
pub use codetether_tetherscript_runtime::Host;
pub use tool::TetherScriptPluginTool;
