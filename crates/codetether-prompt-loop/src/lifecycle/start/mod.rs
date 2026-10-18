//! Ordered preparation and notification before a provider step.
//! [run] checks execution, drains steering, prepares the model and publishes
//! thinking before enforcing the cost budget. Native operations use [Host].
mod host;
mod run;
pub use host::Host;
pub use run::run;
