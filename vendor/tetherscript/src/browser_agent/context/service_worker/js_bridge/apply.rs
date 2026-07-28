//! Apply drained service-worker bridge operations to the context model.

use super::operations::BridgeOperation;
use crate::browser_agent::context::context_state::SharedContextState;
use crate::browser_agent::context::service_worker::origin;
use crate::browser_agent::page::BrowserPage;

pub(super) fn ops(page: &mut BrowserPage, ops: Vec<BridgeOperation>) -> Result<(), String> {
    if ops.is_empty() {
        return Ok(());
    }
    let origin = origin::service_worker_origin(&page.session.url);
    let Some(state) = page.context_state.clone() else {
        return Err("service worker API requires a page attached to BrowserContext".into());
    };
    for op in ops {
        match op {
            BridgeOperation::Register { scope, script_url } => {
                state
                    .borrow_mut()
                    .service_workers
                    .register(&origin, &scope, &script_url);
            }
            BridgeOperation::CacheStorageDelete { cache_name } => {
                delete_cache(&state, &origin, &cache_name);
            }
            BridgeOperation::CacheDelete {
                cache_name,
                request_url,
            } => {
                let url = origin::service_worker_url(&origin, &request_url);
                state
                    .borrow_mut()
                    .service_workers
                    .cache_delete(&origin, &cache_name, &url);
            }
        }
    }
    Ok(())
}

fn delete_cache(state: &SharedContextState, origin: &str, cache_name: &str) {
    let keys = state
        .borrow()
        .service_workers
        .cache_keys(origin, cache_name);
    for key in keys {
        state
            .borrow_mut()
            .service_workers
            .cache_delete(origin, cache_name, &key);
    }
}
