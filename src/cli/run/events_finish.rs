//! Finish live `codetether run` event capture.

use super::{LiveEvents, drain};
use anyhow::{Context, Result};

impl LiveEvents {
    pub async fn finish_success(self, response: &str) -> Result<()> {
        self.finish(Some(response), None).await
    }

    pub async fn finish_failure(self, error: &str) -> Result<()> {
        self.finish(None, Some(error)).await
    }

    async fn finish(mut self, response: Option<&str>, error: Option<&str>) -> Result<()> {
        self.tx.take();
        self.handle.await.context("join live event sink")??;
        let mut mapper = self.mapper.lock().await;
        let event = match (response, error) {
            (Some(response), None) => mapper.turn_completed(response),
            (None, Some(error)) => mapper.turn_failed(error),
            _ => return Ok(()),
        };
        drain::emit(&self.store, &event, self.jsonl).await
    }
}
