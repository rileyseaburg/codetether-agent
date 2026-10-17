//! Core TetherScript execution dispatch.

mod browser;
mod computer;
mod dispatch;
mod finish;
mod host;
mod host_grants;
mod interp;
mod outcome;
mod parse;
mod unwind;

pub use browser::BrowserGrant;
pub use computer::ComputerGrant;
pub use dispatch::run;
pub use outcome::TetherScriptOutcome;

#[cfg(test)]
#[path = "runner/process_arity_tests.rs"]
mod process_arity_tests;

#[cfg(test)]
#[path = "runner/process_migration_tests.rs"]
mod process_migration_tests;
