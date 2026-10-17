//! PRD file persistence, dependency readiness and stage planning.
//!
//! Methods are implemented on Prd, so callers use its public methods directly.
use crate::{Prd, UserStory};
use std::path::PathBuf;
mod dependencies;
mod load;
mod progress;
mod save;
mod selection;
mod stages;
