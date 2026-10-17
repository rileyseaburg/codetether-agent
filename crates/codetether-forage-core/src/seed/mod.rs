//! Seed default and mission-derived objectives while preserving repository guards.
use crate::{MoonshotRubric, truncate_goal};
use anyhow::Result;
use codetether_okr::{KeyResult, Okr, OkrRepository, OkrStatus};
use uuid::Uuid;
mod default_seed;
mod eligibility;
mod initial;
mod moonshot;
pub use default_seed::seed_default_okr_if_empty;
pub use eligibility::{seed_moonshot_okr_if_empty, seed_moonshot_okr_if_no_opportunities};
pub use initial::seed_initial_okr_if_empty;
pub use moonshot::seed_moonshot_okr;

mod clarified;
pub use clarified::persist_clarified_goals;
