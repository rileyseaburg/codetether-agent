//! Bind context CLI commands to the application's durable sessions.
use crate::{cli::ContextArgs, provider::Message, session::Session};
use anyhow::Result;
#[async_trait::async_trait]
impl codetether_context_cli::ResetSession for Session {
    fn add_message(&mut self, message: Message) {
        Session::add_message(self, message);
    }
    async fn save(&self) -> Result<()> {
        Session::save(self).await
    }
}
/// Execute context commands against the native session store.
/// # Errors
/// Returns session persistence, history browsing or output errors.
pub async fn execute(args: ContextArgs) -> Result<()> {
    codetether_context_cli::execute::<Session>(args).await
}
