//! PR lifecycle policy and default configuration.
//! [advance_lifecycle] maps CI/review observations to [PrLifecycle] states.
use serde::{Deserialize, Serialize};
mod advance;
mod config;
mod state;
pub use advance::advance_lifecycle;
pub use config::AutopilotConfig;
pub use state::PrLifecycle;
