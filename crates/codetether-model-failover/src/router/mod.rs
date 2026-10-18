//! Session failover candidate construction and provider selection.
//!
//! [known_good_router_candidates] combines configured preferences, same-provider
//! alternatives and cross-provider fallbacks. [choose_router_target] checks
//! availability; [choose_router_target_bandit] optionally reorders by delegation
//! score before using the same fallback policy.
mod bandit;
mod candidates;
mod preferences;
mod provider_candidates;
mod target;
pub use bandit::choose_router_target_bandit;
pub use candidates::known_good_router_candidates;
pub use target::choose_router_target;
/// Provider availability needed by the router.
/// # Implementors
/// Applications expose their registry lookup without transferring provider ownership.
pub trait Registry {
    /// Whether a provider can currently be selected by name.
    fn available(&self, name: &str) -> bool;
}
#[cfg(test)]
mod tests;
