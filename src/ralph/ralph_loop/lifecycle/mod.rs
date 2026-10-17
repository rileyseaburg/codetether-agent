//! Application construction, configuration, run lifecycle and persistence wiring.
//!
//! Application adapters retain live resources and call shared Ralph services.
//! Methods preserve their original visibility within the Ralph loop owner.
use super::*;
mod configure;
mod construct;
mod lifecycle_cleanup;
mod lifecycle_finish;
mod lifecycle_record;
mod lifecycle_start;
mod persistence;
mod progress;
mod run;
mod status;
