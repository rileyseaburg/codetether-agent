//! Resolve concurrent creation and evict the least recently used manager.
use super::state::{MAX_LSP_MANAGERS, Managers};
use codetether_lsp_core::manager::{LspManager, ManagedClient};
use std::sync::Arc;
pub(super) async fn select<C: ManagedClient>(
    cell: &Managers<C>,
    key: String,
    access: u64,
    manager: Arc<LspManager<C>>,
) -> (Arc<LspManager<C>>, Option<Arc<LspManager<C>>>) {
    let evicted_manager = {
        let mut guard = cell.write().await;
        if let Some((last_access, existing_manager)) = guard.get_mut(&key) {
            *last_access = access;
            return (Arc::clone(existing_manager), None);
        }

        let evicted_manager = if guard.len() >= MAX_LSP_MANAGERS {
            let evicted_key = guard
                .iter()
                .min_by_key(|(_, (last_access, _))| *last_access)
                .map(|(evicted_key, _)| evicted_key.clone());
            evicted_key
                .and_then(|evicted_key| guard.remove(&evicted_key))
                .map(|(_, evicted_manager)| evicted_manager)
        } else {
            None
        };

        guard.insert(key, (access, Arc::clone(&manager)));
        evicted_manager
    };
    (manager, evicted_manager)
}
