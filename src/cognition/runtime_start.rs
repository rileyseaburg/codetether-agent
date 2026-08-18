//! Starting the perpetual cognition loop.

use anyhow::{Result, anyhow};
use chrono::Utc;
use std::sync::atomic::Ordering;

use super::loop_ctx::LoopCtx;
use super::{
    CognitionRuntime, CognitionStatus, CreatePersonaRequest, StartCognitionRequest, defaults,
    loop_drive,
};

/// Minimum loop interval, in milliseconds.
const MIN_INTERVAL_MS: u64 = 100;

impl CognitionRuntime {
    /// Start the perpetual cognition loop.
    ///
    /// Seeds a root persona when none exist. Calling this while already running
    /// is a no-op that returns current status.
    ///
    /// # Errors
    ///
    /// Returns an error when cognition is disabled, or when seeding the initial
    /// persona fails.
    pub async fn start(&self, req: Option<StartCognitionRequest>) -> Result<CognitionStatus> {
        if !self.enabled {
            return Err(anyhow!(
                "Perpetual cognition is disabled. Set CODETETHER_COGNITION_ENABLED=true."
            ));
        }
        let seed = self.apply_start_request(req).await;
        if self.personas.read().await.is_empty() {
            self.create_persona(seed.unwrap_or_else(defaults::default_seed_persona))
                .await?;
        }
        if self.running.load(Ordering::SeqCst) {
            return Ok(self.status().await);
        }

        self.running.store(true, Ordering::SeqCst);
        *self.started_at.write().await = Some(Utc::now());

        let ctx = LoopCtx::from_runtime(self);
        *self.loop_handle.lock().await = Some(tokio::spawn(loop_drive::drive(ctx)));
        Ok(self.status().await)
    }

    /// Apply the interval override and return the requested seed persona.
    async fn apply_start_request(
        &self,
        req: Option<StartCognitionRequest>,
    ) -> Option<CreatePersonaRequest> {
        let req = req?;
        if let Some(interval) = req.loop_interval_ms {
            *self.loop_interval_ms.write().await = interval.max(MIN_INTERVAL_MS);
        }
        req.seed_persona
    }
}
