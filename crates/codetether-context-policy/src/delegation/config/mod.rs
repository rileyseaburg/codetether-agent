//! delegation::config contracts and policy operations.
//! Delegation config tunables and defaults.

use serde::{Deserialize, Serialize};

/// Default uncertainty penalty `γ` for LCB scoring (CADMAS-CTX §3.4).
pub const DEFAULT_GAMMA: f64 = 0.5;

/// Default delegation margin `δ`.
pub const DEFAULT_DELTA: f64 = 0.05;

/// Default weak-prior strength `κ`.
pub const DEFAULT_KAPPA: f64 = 2.0;

/// Default forgetting factor `λ` (1.0 = disabled).
pub const DEFAULT_LAMBDA: f64 = 1.0;
mod fn_default_gamma;
use fn_default_gamma::default_gamma;
mod fn_default_delta;
use fn_default_delta::default_delta;
mod fn_default_kappa;
use fn_default_kappa::default_kappa;
mod fn_default_lambda;
use fn_default_lambda::default_lambda;
mod type_delegation_config;
pub use type_delegation_config::DelegationConfig;
mod impl_default_default_0;
