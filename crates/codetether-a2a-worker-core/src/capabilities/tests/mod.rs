//! Capability registration regression tests.

use super::worker_capabilities;
mod environment;
use environment::with_worker_capability_env;
mod worker_capabilities_include_env_additions_without_duplicates;
mod worker_capabilities_include_knative_when_enabled;
