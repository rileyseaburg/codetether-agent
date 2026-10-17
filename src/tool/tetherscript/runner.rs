//! Core TetherScript execution dispatch.

mod browser;
mod computer;
mod computer_invoke;
mod computer_lists;
mod computer_narrow;
mod computer_origin;
mod computer_payload;
mod computer_policy;
mod computer_scopes;
mod computer_value;
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
