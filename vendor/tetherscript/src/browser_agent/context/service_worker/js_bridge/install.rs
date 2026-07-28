//! Service-worker bridge script assembly.

use super::{script, script_data};
use crate::browser_agent::context::service_worker::origin;
use crate::browser_agent::page::BrowserPage;

pub(super) fn script(page: &BrowserPage) -> String {
    let origin = origin::service_worker_origin(&page.session.url);
    let Some(state) = page.context_state.as_ref() else {
        return script::install(&origin, "[]", "[]");
    };
    let state = state.borrow();
    let registrations = state.service_workers.registrations(&origin);
    let caches = state.service_workers.cache_records();
    let caches = caches
        .into_iter()
        .filter(|record| record.origin == origin)
        .collect::<Vec<_>>();
    script::install(
        &origin,
        &script_data::registrations(&registrations),
        &script_data::caches(&caches, &origin),
    )
}
