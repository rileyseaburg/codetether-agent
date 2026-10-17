//! default policy implementation.
use super::*;

impl Default for DelegationConfig {
    fn default() -> Self {
        Self {
            gamma: DEFAULT_GAMMA,
            delta: DEFAULT_DELTA,
            kappa: DEFAULT_KAPPA,
            lambda: DEFAULT_LAMBDA,
            enabled: false,
        }
    }
}
