//! Publish lifecycle events after the shared reap mutation releases its lock.
use super::CognitionRuntime;
use crate::persona::lifecycle::{prepare_reap, reaped_event};
use crate::{ReapPersonaRequest, status::ReapPersonaResponse};
use anyhow::Result;

impl<T, R> CognitionRuntime<T, R> {
    /// Reap one persona or its full descendant tree.
    ///
    /// # Errors
    ///
    /// Returns an error when `persona_id` is unknown.
    pub async fn reap_persona(
        &self,
        persona_id: &str,
        req: ReapPersonaRequest,
    ) -> Result<ReapPersonaResponse> {
        let plan = prepare_reap(&self.personas, persona_id, req).await?;
        for id in &plan.reaped_ids {
            self.push_event(reaped_event(id, &plan.reason, plan.cascade, plan.now))
                .await;
        }
        Ok(ReapPersonaResponse {
            count: plan.reaped_ids.len(),
            reaped_ids: plan.reaped_ids,
        })
    }
}
