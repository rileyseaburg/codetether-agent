//! Decode bus events from SSE text lines.
use super::*;
impl BusBridge {
    /// Process a single SSE text line.
    pub(super) async fn process_sse_line(
        &self,
        line: &str,
        event_type: &mut String,
        data_buf: &mut String,
    ) {
        if line.is_empty() {
            // Empty line = end of event
            if event_type == "bus" && !data_buf.is_empty() {
                match serde_json::from_str::<BusEnvelope>(data_buf) {
                    Ok(envelope) => {
                        self.push_envelope(envelope).await;
                    }
                    Err(e) => {
                        tracing::debug!(target: "codetether_agent::mcp::bus_bridge", error = %e, "BusBridge: failed to parse bus envelope");
                    }
                }
            }
            event_type.clear();
            data_buf.clear();
        } else if let Some(rest) = line.strip_prefix("event:") {
            *event_type = rest.trim().to_string();
        } else if let Some(rest) = line.strip_prefix("data:") {
            if !data_buf.is_empty() {
                data_buf.push('\n');
            }
            data_buf.push_str(rest.trim());
        }
        // Ignore comment lines (`:`) and other prefixes
    }
}
