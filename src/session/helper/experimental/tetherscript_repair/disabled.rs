//! No-op fallback when tetherscript feature is not enabled.

use super::super::ExperimentalStats;
use crate::provider::Message;

pub fn repair_reasoning(_messages: &mut Vec<Message>) -> ExperimentalStats {
    ExperimentalStats::default()
}
