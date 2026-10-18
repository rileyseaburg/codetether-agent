//! Native specialization of shared best-effort thinker startup.
use super::{ThinkerClient, ThinkerConfig};
use std::sync::Arc;
pub(super) fn init_thinker(config: Option<ThinkerConfig>) -> Option<Arc<ThinkerClient>> {
    codetether_cognition_core::thinker::client::initialize(config)
}
