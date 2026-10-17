//! Quality-gate sequencing with host shell execution and result notifications.
//!
//! run_quality_gates locates the Rust workspace root and stops at the first
//! failed configured command. Hosts retain policy and user-interface wiring.
mod failure;
mod find_root;
mod run;
pub use find_root::find_cargo_root;
pub use run::run_quality_gates;
