//! Registry new operation.
use crate::ToolRegistry;
use std::{collections::HashMap, sync::Arc};
impl ToolRegistry {
    /// # Arguments
    /// No arguments; creates an independent registry.
    /// # Returns
    /// An empty registry.
    /// # Examples
    /// ```no_run
    /// let _registry = codetether_tool_discovery::ToolRegistry::new();
    /// ```
    pub fn new() -> Self {
        Self {
            tools: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }
}
