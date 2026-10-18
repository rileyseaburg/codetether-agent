//! Browser-control CLI argument schemas, request mapping and output rendering.
//!
//! [execute] translates [BrowserCtlArgs] into the shared browser tool protocol
//! or offline helpers. Browser lifecycle and execution belong to those crates.
mod args;
mod offline;
mod offline_args;
mod request;
mod run;

pub use args::BrowserCtlArgs;
mod command;
pub use command::BrowserCtlCommand;
pub use offline_args::OfflineCommand;
pub use run::execute;

#[cfg(test)]
mod tests;

mod request_attach;
