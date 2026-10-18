//! Bind shared parallel recording to the application's session and routing.
use crate::provider::{Message, Provider};
use crate::session::Session;
use async_trait::async_trait;
use codetether_tool_parallel::Output;
use std::{path::Path, sync::Arc};
pub(super) struct NativeHost<'a> {
    pub session: &'a mut Session,
    pub cwd: &'a Path,
    pub model: &'a str,
    pub provider: Arc<dyn Provider>,
}
#[async_trait]
impl codetether_tool_parallel::Host for NativeHost<'_> {
    fn id(&self) -> &str {
        &self.session.id
    }
    fn enrich(&self, input: &serde_json::Value) -> serde_json::Value {
        crate::session::helper::runtime::enrich_tool_input_for_session(
            input,
            self.cwd,
            self.session,
        )
    }
    async fn save(&self) -> anyhow::Result<()> {
        self.session.save().await
    }
    fn route(&self, output: &Output) -> String {
        super::route::route(self.session, self.model, Arc::clone(&self.provider), output)
    }
    fn record(&mut self, message: Message) {
        self.session.add_message(message);
    }
}
