//! Identity reaching every provider dispatch path, without a network request.
use super::{request, text};
use crate::provider::Provider;
use crate::provider::metrics::MetricsProvider;
use std::sync::{Arc, Mutex};

#[path = "dispatch/aliases.rs"]
mod aliases;
#[path = "dispatch/mock.rs"]
mod mock;
#[path = "dispatch/paths.rs"]
mod paths;
#[path = "dispatch/registry.rs"]
mod registry;
#[path = "dispatch/rewrapping.rs"]
mod rewrapping;
