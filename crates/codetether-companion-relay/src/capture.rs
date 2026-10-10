//! Capture admission: budget, cooldown, and fresh-request matching.
use crate::{ApiError, admission, events::event, runtime::Runtime};
use codetether_companion_protocol::{Capture, EventKind};
use tokio_util::sync::CancellationToken;

/// Work admitted for one analysis run.
pub(crate) struct Admitted {
    pub generation: u64,
    pub cancel: CancellationToken,
    pub prompt: String,
    pub type_requested: bool,
}
impl Runtime {
    /// Admit a validated frame or reject it with the relay's 409/410 rules.
    pub(crate) fn admit(
        &mut self,
        frame: &Capture,
        captured: i64,
        now: i64,
    ) -> Result<Admitted, ApiError> {
        admission::check(self, frame, captured, now)?;
        self.generation += 1;
        let cancel = CancellationToken::new();
        self.active = Some((self.generation, cancel.clone()));
        self.last_at = now;
        self.frames += 1;
        let type_requested = self
            .pending
            .as_ref()
            .is_some_and(|p| crate::model_typing::requests_typing(&p.question));
        let prompt = self
            .pending
            .take()
            .map_or_else(|| self.prompt.clone(), |p| p.question);
        self.text.clear();
        self.status = "analyzing".into();
        self.captured_at = Some(frame.captured_at.clone());
        let mut started = event(EventKind::Capture, None, Some("analyzing"));
        started.captured_at = self.captured_at.clone();
        self.publish(started);
        Ok(Admitted {
            generation: self.generation,
            cancel,
            prompt,
            type_requested,
        })
    }
}
