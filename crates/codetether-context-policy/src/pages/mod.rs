//! Typed context pages and minimum-fidelity policy.
//!
//! [classify] produces a [PageKind]. Each kind specifies a minimum
//! [ResidencyLevel] and the permitted sequence of degradation states.
use codetether_provider_core::{ContentPart, Message, Role};
use serde::{Deserialize, Serialize};
mod types;
pub use types::{PageKind, ResidencyLevel};
mod classify;
mod fidelity;
pub use classify::{classify, classify_all};
#[cfg(test)]
mod tests;
