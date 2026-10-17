//! Training projection and grouping regressions.
use super::*;
use chrono::Utc;
use serde_json::Value;
mod tool_request_envelope;
use tool_request_envelope::tool_request_envelope;
mod tool_response_envelope;
use tool_response_envelope::tool_response_envelope;
mod test_heartbeat_skipped_role;
mod test_tool_grouping_keeps_different_correlation_ids_separate;
mod test_tool_grouping_keeps_legacy_none_correlation_compatible;
mod test_training_record_agent_message;
mod test_training_record_tool_request;
mod test_training_record_tool_response;
