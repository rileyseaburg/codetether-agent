//! Same-turn and queued delivery for inter-agent messages.

#[path = "communication/active.rs"]
mod active;
#[path = "communication/idle.rs"]
mod idle;
#[path = "communication/route.rs"]
mod route;
#[path = "communication/wait.rs"]
mod wait;

pub(crate) use active::steer;
pub(crate) use idle::queue_only;
pub(crate) use route::Route;

#[cfg(test)]
#[path = "communication/active_tests.rs"]
mod active_tests;
#[cfg(test)]
#[path = "communication/idle_tests.rs"]
mod idle_tests;
