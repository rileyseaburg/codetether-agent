//! Native model construction under the existing serialized handle.
use crate::ThinkerConfig;
use anyhow::Result;
use codetether_cognition_native::CandleThinker;
use std::sync::{Arc, Mutex};
pub(super) fn native(config: &ThinkerConfig) -> Result<Arc<Mutex<CandleThinker>>> {
    Ok(Arc::new(Mutex::new(CandleThinker::new(config)?)))
}
