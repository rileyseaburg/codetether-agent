//! Application construction, configuration, run lifecycle and persistence wiring.
//!
//! Application adapters retain live resources and call shared Ralph services.
//! Methods preserve their original visibility within the Ralph loop owner.
use super::*;
mod configure;
mod construct;
mod host;
mod persistence;
mod progress;
mod run;
mod status;
