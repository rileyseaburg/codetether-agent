//! Introduction delivery and ledger access using application configuration.
//!
//! [`ledger`] resolves the standard data-directory ledger; [`send`] applies
//! the same location to bounded delivery. Detection and reply policy stay shared.
pub mod ledger;
mod ledger_path;
pub mod send;
#[cfg(test)]
mod tests;
