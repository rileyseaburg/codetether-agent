//! Stopping the perpetual cognition loop.

use anyhow::Result;
use chrono::Utc;
use serde_json::json;
use std::sync::atomic::Ordering;
use uuid::Uuid;

use super::{CognitionRuntime, CognitionStatus, ThoughtEvent, ThoughtEventType};

impl CognitionRuntime {
    /// Stop the loop, abort its task, and record an optional reason.
    ///
    /// # Errors
    ///
    /// Returns an error only if status collection fails.
    pub async fn stop(&self, reason: Option<String>) -> Result<CognitionStatus> {
        self.running.store(false, Ordering::SeqCst);

        if let Some(handle) = self.loop_handle.lock().await.take() {
            handle.abort();
            let _ = handle.await;
        }

        if let Some(reason) = reason {
            self.push_event(ThoughtEvent {
                id: Uuid::new_v4().to_string(),
                event_type: ThoughtEventType::CheckResult,
                persona_id: None,
                swarm_id: None,
                timestamp: Utc::now(),
                payload: json!({ "stopped": true, "reason": reason }),
            })
            .await;
        }

        Ok(self.status().await)
    }
}
