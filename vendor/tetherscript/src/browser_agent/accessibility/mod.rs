//! Deterministic accessibility snapshots for agent-controlled pages.
//!
//! This module builds compact agent accessibility metadata from the
//! browser-agent DOM without owning layout, network, or script runtime state.

mod build;
mod focus;
mod model;
mod page;
mod state;
mod visibility;

pub use model::{AccessibilityNode, AccessibilitySnapshot, AccessibilityState};

#[cfg(test)]
mod focus_tests;
#[cfg(test)]
mod names_tests;
#[cfg(test)]
mod state_tests;
