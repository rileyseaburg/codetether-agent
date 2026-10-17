//! Process-wide system allocator controls.
//!
//! Applies early glibc arena limits and exposes explicit heap reclamation.

mod config;
#[cfg(test)]
mod config_tests;
mod reclaim;
mod tune;

pub(super) fn initialize_early() {
    tune::initialize();
}

pub(super) fn configure() {
    tune::configure(config::Settings::load());
}

pub(super) fn trim() -> bool {
    reclaim::trim()
}
