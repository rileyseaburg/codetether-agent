#![allow(dead_code)]
//! Self-healing selector support for browser agents.

pub mod candidate;
pub mod fingerprint;
pub mod generator;
pub mod healer;
pub mod health;
pub mod ranking;
pub mod strategies;

#[cfg(test)]
mod tests;

pub use candidate::{ElementPath, HealStrategy, SelectorCandidate};
pub use fingerprint::{DomFingerprint, DomNode};
pub use generator::SelectorGenerator;
pub use healer::SelfHealingSelector;
pub use health::{SelectorHealth, SelectorRecommendation};
pub use ranking::rank_candidates;
