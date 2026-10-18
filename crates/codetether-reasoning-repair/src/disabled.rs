//! No-op fallback when tetherscript feature is not enabled.

use codetether_context_policy::transforms::ExperimentalStats;
use codetether_provider_core::Message;

pub fn repair_reasoning(_messages: &mut Vec<Message>) -> ExperimentalStats {
    ExperimentalStats::default()
}
