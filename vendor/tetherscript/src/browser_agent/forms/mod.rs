//! Form-control actions for agent-driven pages.
//!
//! This module adds agent state actions for checkable controls and
//! select boxes without changing locator or retry internals.

mod actions;
mod controls;
mod options;
mod ready;
mod report;
mod scripts;
mod select_hit;
mod select_ready;

#[cfg(test)]
mod check_tests;
#[cfg(test)]
mod select_tests;
