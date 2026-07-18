//! Durable child loading with bounded LRU residency.

#[path = "residency/activate.rs"]
mod activate;
#[path = "residency/capacity.rs"]
mod capacity;
#[path = "residency/ensure.rs"]
mod ensure;
#[path = "residency/evict.rs"]
mod evict;
#[path = "residency/order.rs"]
mod order;
#[path = "residency/outcome.rs"]
mod outcome;
#[path = "residency/overlay.rs"]
mod overlay;
#[path = "residency/resume_config.rs"]
mod resume_config;
#[path = "residency/singleflight.rs"]
mod singleflight;
#[path = "residency/slot.rs"]
mod slot;
#[path = "residency/stale.rs"]
mod stale;
#[path = "residency/state.rs"]
mod state;

pub(in crate::tool::agent) use capacity::reserve;
pub(in crate::tool) use ensure::open;
pub(crate) use order::touch;
pub(crate) use outcome::EnsureOpen;
pub(in crate::tool) use resume_config::ResumeConfig;
pub(in crate::tool::agent) use singleflight::acquire as acquire_transition;

pub(in crate::tool::agent) fn forget(agent_id: &str) {
    order::forget(agent_id);
    stale::forget(agent_id);
}

pub(in crate::tool::agent) use stale::mark as mark_stale;

#[cfg(test)]
include!("residency/test_modules.rs");
