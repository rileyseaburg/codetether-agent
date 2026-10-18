//! Process and optional-service startup for CLI applications.
//!
//! After native early-command handling, call [environment::initialize] before
//! parsing the full CLI. [logging::initialize] configures output destinations;
//! [crash::initialize] and [vault::initialize] prepare optional services after
//! command selection. The application supplies its own version and memory guards.
pub mod crash;
pub mod environment;
pub mod lifecycle;
pub mod logging;
pub mod vault;
