//! Self-healing selector support.

#![allow(dead_code)]

pub mod cache;
pub mod extract;
pub mod score;
pub mod search;

#[allow(unused_imports)]
pub use cache::SelectorHealCache;
#[allow(unused_imports)]
pub use extract::{extract_props, ElementLike};
#[allow(unused_imports)]
pub use score::{element_similarity, ElementProps};
#[allow(unused_imports)]
pub use search::heal_selector;

#[cfg(test)]
mod tests;
