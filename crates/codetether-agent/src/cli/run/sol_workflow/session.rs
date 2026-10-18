//! Application session fields used by the native Sol worker.
use crate::{agent::ToolUse, config::Config, session::Session};
impl codetether_native_cli::sol::Session for Session {
    fn set_agent_name(&mut self, name: &str) {
        self.set_agent_name(name);
    }
    fn apply_config(&mut self, config: &Config) {
        self.apply_config(config, None);
    }
    fn model(&self) -> String {
        self.metadata.model.clone().unwrap_or_default()
    }
    fn tool_uses(&self) -> &[ToolUse] {
        &self.tool_uses
    }
}
