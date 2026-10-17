//! Shut down and clear the shared manager cache.
use super::{Host, LspTool};
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
impl<H: Host> LspTool<H> {
    /// Shutdown all LSP clients, releasing resources.
    #[allow(dead_code)]
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// Nothing after requesting shutdown of all cached managers.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example<H: codetether_lsp_tool::Host>() {
    /// codetether_lsp_tool::LspTool::<H>::new().shutdown_all().await;
    /// # }
    /// ```
    pub async fn shutdown_all(&self) {
        let cell = H::state()
            .managers
            .get_or_init(|| Arc::new(RwLock::new(HashMap::new())));
        let managers = {
            let mut guard = cell.write().await;
            let managers = guard
                .values()
                .map(|(_, manager)| Arc::clone(manager))
                .collect::<Vec<_>>();
            guard.clear();
            managers
        };
        for manager in managers {
            manager.shutdown_all().await;
        }
    }
}
