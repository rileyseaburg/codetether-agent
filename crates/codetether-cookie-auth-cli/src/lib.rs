//! Provider cookie import from Netscape cookie files into the shared Vault store.
//!
//! [CookieAuthArgs] selects the provider and filtering policy. [execute] validates
//! and parses the file, selects live cookies, stores expiry metadata and prints
//! the existing import report.
mod args;
mod execute;
mod expiry;
mod metadata;
mod metadata_expiry;
mod names;
mod parse;
mod render;
mod report;
mod row;
mod save;
mod select;
pub use args::CookieAuthArgs;
pub use execute::execute;
#[cfg(test)]
mod tests;
