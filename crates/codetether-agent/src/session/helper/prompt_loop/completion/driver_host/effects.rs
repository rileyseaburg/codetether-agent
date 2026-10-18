//! Native delegation, usage, checkpoint and recovery bindings.
use super::super::super::Runner;
use crate::{provider::CompletionResponse, session::Bucket};
use codetether_prompt_loop::{
    LoopState,
    completion::{Attempt, driver::Effects},
};
use std::time::Duration;
impl Effects for Runner<'_> {
    fn succeeded(&mut self, bucket: Bucket) {
        self.inner.session.metadata.delegation.update(
            &self.inner.model.provider_name,
            crate::session::delegation_skills::MODEL_CALL,
            bucket,
            true,
        );
    }
    async fn usage(&mut self, response: &CompletionResponse, elapsed: Duration) {
        super::super::super::response::usage(self, response, elapsed).await;
    }
    fn progress(&mut self) -> &mut LoopState {
        &mut self.inner.progress
    }
    async fn checkpoint(&mut self, error: &anyhow::Error) -> anyhow::Result<()> {
        super::super::super::super::prompt_call::persist_stream_checkpoints(
            self.inner.session,
            error,
        )
        .await
    }
    async fn recover(
        &mut self,
        step: usize,
        attempt: &mut Attempt,
        error: &anyhow::Error,
    ) -> anyhow::Result<bool> {
        super::super::recovery::recover(self, step, attempt, error).await
    }
}
