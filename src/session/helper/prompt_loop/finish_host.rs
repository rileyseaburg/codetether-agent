//! Native storage and final-answer services.
use super::super::Runner;
use crate::session::{Session, SessionEvent};
use tokio::sync::mpsc;
impl codetether_prompt_loop::lifecycle::finish::Host for Runner<'_> {
    type Snapshot = Session;
    async fn save(&mut self) -> anyhow::Result<()> {
        self.inner.session.save().await
    }
    async fn archive(&mut self) {
        super::super::super::archive::archive_event_stream_to_s3(
            &self.inner.session.id,
            super::super::super::archive::event_stream_path(),
        )
        .await;
    }
    fn events(&self) -> Option<&mpsc::Sender<SessionEvent>> {
        self.inner.events.as_ref()
    }
    fn final_text(&self) -> String {
        super::super::super::evidence::gate_final_answer(
            self.inner.progress.output.trim(),
            self.inner.session,
        )
    }
    fn session_id(&self) -> &str {
        &self.inner.session.id
    }
}
