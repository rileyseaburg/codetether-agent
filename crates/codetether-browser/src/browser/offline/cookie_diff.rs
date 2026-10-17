//! Cookie-jar comparison by name, domain, and path.
//!
//! [diff] compares records in memory; [run] reads and serializes a comparison.
//! [CookieDelta] reports all differences and duplicate keys.
mod compare;
mod index;
mod model;
mod read;
pub use compare::diff;
pub use model::{Changed, CookieDelta};
pub use read::run;

#[cfg(test)]
mod tests;
