//! Explicit Rust development commands for CLI, editor, and native-agent callers.
//!
//! [`plan`] creates a shell-free argv without I/O; [`run`] checks prerequisites
//! and executes it. Nothing installs tools or changes global Cargo configuration.
//! ra-jit accelerates builds, not editor analysis, and lacks a borrow checker.
//!
//! ```
//! use codetether_rust_build::{RustArgs, RustAction, plan};
//! let args = RustArgs { action: RustAction::Check, ..Default::default() };
//! assert_eq!(plan(&args).unwrap().args, ["+stable", "check"]);
//! ```
mod action;
mod args;
mod cache;
mod doctor;
mod plan;
mod process;
#[cfg(test)]
mod process_tests;
mod runner;
mod validate;

pub use action::RustAction;
pub use args::RustArgs;
pub use plan::{Plan, plan};
pub use runner::run;
