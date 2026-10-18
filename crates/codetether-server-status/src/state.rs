//! Caller-owned immutable configuration and application identity.
use codetether_config::Config;
use std::sync::Arc;
#[derive(Clone)]
pub(super) struct StatusState {
    pub config: Arc<Config>,
    pub version: &'static str,
    pub name: &'static str,
}
