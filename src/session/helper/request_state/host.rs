//! Native registry and session hooks for shared provider request preparation.
use crate::provider::{Provider, ToolDefinition};
use crate::session::Session;
use crate::tool::ToolRegistry;
use std::{path::Path, sync::Arc};
pub(super) struct NativeHost<'a> {
    pub provider: Arc<dyn Provider>,
    pub model: &'a str,
    pub session: &'a Session,
}
impl codetether_request_preparation::Host for NativeHost<'_> {
    type Registry = ToolRegistry;
    fn context(&self) -> (std::path::PathBuf, bool, bool) {
        super::session_context::resolve(self.session)
    }
    fn registry(&self, cwd: &Path, prior_context: bool, autonomous: bool) -> Arc<ToolRegistry> {
        super::super::workspace_tools::registry_for_cwd(
            self.provider.clone(),
            self.model,
            cwd,
            prior_context,
            autonomous,
        )
    }
    fn active(&self, registry: &ToolRegistry, provider: &str) -> Vec<ToolDefinition> {
        super::tools::active_tool_definitions(registry, provider)
    }
    fn advertised(
        &self,
        supported: bool,
        tools: &[ToolDefinition],
        provider: &str,
    ) -> Vec<ToolDefinition> {
        super::tools::advertised_tools(supported, tools, provider)
    }
    fn compose(&self, prompt: &str) -> String {
        crate::session::tasks::runtime::compose(prompt, &self.session.id)
    }
}
