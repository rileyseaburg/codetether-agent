//! Mock host that detects project context or task injection into general chat.
use codetether_provider_core::ToolDefinition;
use codetether_request_preparation::Host;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
pub struct Fixture {
    pub cwd: PathBuf,
    pub general: bool,
}
impl Host for Fixture {
    type Registry = ();
    fn general_chat(&self) -> bool {
        self.general
    }
    fn context(&self) -> (PathBuf, bool, bool) {
        assert!(
            !self.general,
            "Chat must not load persisted project context"
        );
        (self.cwd.clone(), true, true)
    }
    fn registry(&self, cwd: &Path, prior: bool, autonomous: bool) -> Arc<()> {
        if self.general {
            assert_ne!(cwd, self.cwd);
            assert!(!prior && !autonomous);
        } else {
            assert_eq!(cwd, self.cwd);
        }
        Arc::new(())
    }
    fn active(&self, _: &(), _: &str) -> Vec<ToolDefinition> {
        vec![codetether_tool_catalog::list_tools_bootstrap_definition()]
    }
    fn advertised(&self, native: bool, tools: &[ToolDefinition], _: &str) -> Vec<ToolDefinition> {
        if native { tools.to_vec() } else { vec![] }
    }
    fn compose(&self, prompt: &str) -> String {
        assert!(
            !self.general,
            "Chat must not inherit repository task instructions"
        );
        format!("{prompt}\nCODING_TASK_ONLY")
    }
}
#[path = "support/repository.rs"]
mod repository;
pub use repository::repository;