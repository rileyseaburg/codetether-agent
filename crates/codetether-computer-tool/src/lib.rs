//! Computer-use tool assembly over dedicated platform and worker crates.
//!
//! [ComputerUseTool] implements the shared tool contract. Windows requests use
//! subprocess isolation; other platforms dispatch directly. [worker::serve]
//! binds the child protocol to direct [platform::dispatch] without recursion.
pub use codetether_computer_contract::{input, response, schema};
mod execution;
pub mod platform;
mod tool;
pub mod worker;
pub use tool::ComputerUseTool;
