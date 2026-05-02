//! CADMAS-CTX delegation calibration (arXiv:2604.17950).
//!
//! Hierarchical per-(agent, skill, bucket) Beta posteriors scored
//! under a risk-aware LCB. Replaces static per-agent skill scores
//! which have provably linear regret.
//!
//! ## Module layout
//!
//! * [`beta`] — `BetaPosterior` math + update
//! * [`config`] — `DelegationConfig` tunables
//! * [`state`] — `DelegationState` sidecar + scoring/delegation API
//! * [`env`] — `CODETETHER_DELEGATION_ENABLED` override

pub mod beta;
pub mod config;
pub mod env;
pub mod lambda;
pub mod state;
pub mod state_mut;
pub mod state_query;

pub use beta::BetaPosterior;
pub use config::{DEFAULT_DELTA, DEFAULT_GAMMA, DEFAULT_KAPPA, DEFAULT_LAMBDA, DelegationConfig};
pub use state::DelegationState;

#[cfg(test)]
mod tests_beta;
#[cfg(test)]
mod tests_query;
#[cfg(test)]
mod tests_state;
