//! delegation::env contracts and policy operations.
//! Environment variable override for delegation feature flag.

use std::env;
mod fn_env_enabled_override;
pub use fn_env_enabled_override::env_enabled_override;
