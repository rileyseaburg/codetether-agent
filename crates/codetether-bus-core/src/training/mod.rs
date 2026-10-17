//! Convert bus envelopes into chronological JSONL training records.
//!
//! [serialize_envelopes] filters duplicate telemetry, pairs tool calls with
//! their responses, and preserves envelope provenance. [catalog] formats
//! tool schemas for the system messages in those records.
use crate::{BusEnvelope, BusMessage};
use codetether_a2a_core::{Artifact, Part, TaskState};
use serde::{Deserialize, Serialize};
mod record;
use record::TrainingRecord;
mod tool_call;
use tool_call::TrainingToolCall;
mod function;
use function::TrainingFunction;
mod metadata;
use metadata::TrainingMetadata;
mod step;
use step::envelope_step;
mod render;
mod serialize;
use render::envelope_to_training_record;
mod format;
mod provenance;
use format::{agent_message_text, bus_message_kind, parts_to_text};
/// Tool schemas rendered as training system-message content.
pub mod catalog;
mod collect;
mod group;
mod merge;
mod order;
mod speech;
#[cfg(test)]
mod tests;
/// Convert one bus batch into ordered JSON strings, one per training record.
///
/// # Arguments
/// * `envelopes` - Live envelopes belonging to one archival batch.
///
/// # Returns
/// JSON records with incomplete tool groups and duplicate telemetry omitted.
///
/// # Examples
/// ```
/// use codetether_bus_core::training::serialize_envelopes;
/// assert!(serialize_envelopes(&[]).is_empty());
/// ```
pub fn serialize_envelopes(envelopes: &[BusEnvelope]) -> Vec<String> {
    serialize::serialize_training_records(&collect::collect(envelopes))
}
