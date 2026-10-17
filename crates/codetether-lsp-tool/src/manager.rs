//! Reuse cached managers and release evicted idle managers.
use super::{Host, LspTool};
use codetether_lsp_core::manager::LspManager;
use std::{
    collections::HashMap,
    sync::{Arc, atomic::Ordering},
};
use tokio::sync::RwLock;
impl<H: Host> LspTool<H> {
    /// Get or initialize this workspace manager.
    ///
    /// # Arguments
    /// None.
    /// # Returns
    /// The shared manager for this workspace and host.
    /// # Examples
    /// ```rust,no_run
    /// # async fn example<H: codetether_lsp_tool::Host>() {
    /// let _manager = codetether_lsp_tool::LspTool::<H>::new().get_manager().await;
    /// # }
    /// ```
    pub async fn get_manager(&self) -> Arc<LspManager<H::Client>> {
        let access = H::state().access.fetch_add(1, Ordering::Relaxed);
        let key = self.manager_key();
        let cell = H::state()
            .managers
            .get_or_init(|| Arc::new(RwLock::new(HashMap::new())));

        {
            let mut guard = cell.write().await;
            if let Some((last_access, manager)) = guard.get_mut(&key) {
                *last_access = access;
                return Arc::clone(manager);
            }
        }

        let manager = super::manager_create::create(self).await;
        let (manager, evicted_manager) =
            super::manager_select::select(cell, key, access, manager).await;
        if let Some(evicted_manager) = evicted_manager
            && Arc::strong_count(&evicted_manager) == 1
        {
            evicted_manager.shutdown_all().await;
        }

        manager
    }
}
