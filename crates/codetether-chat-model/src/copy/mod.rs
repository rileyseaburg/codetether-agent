//! Select raw retained payloads for clipboard copying.
//!
//! [target] parses a copy command and [latest] searches newest messages first.
mod latest;
mod target;
pub use latest::latest;
pub use target::{Target, target};
#[cfg(test)]
#[path = "tests.rs"]
mod tests;
