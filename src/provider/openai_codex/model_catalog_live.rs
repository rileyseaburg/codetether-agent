//! `'static` view of the latest discovered Codex model slugs.
//!
//! Failover tables hand out `&'static [&'static str]`. The discovered slug
//! list is leaked once per distinct catalog (a handful of short strings per
//! process), so repeated calls with an unchanged catalog allocate nothing.

use std::sync::Mutex;

static LEAKED: Mutex<Option<(Vec<String>, &'static [&'static str])>> = Mutex::new(None);

/// Latest discovered listed slugs, or `None` before any discovery.
pub(super) fn current() -> Option<&'static [&'static str]> {
    let slugs = super::super::model_discovery::discovered_slugs()?;
    let mut slot = LEAKED.lock().ok()?;
    if let Some((cached, leaked)) = slot.as_ref()
        && *cached == slugs
    {
        return Some(leaked);
    }
    let leaked: Vec<&'static str> = slugs
        .iter()
        .map(|slug| &*Box::leak(slug.clone().into_boxed_str()))
        .collect();
    let leaked: &'static [&'static str] = Box::leak(leaked.into_boxed_slice());
    *slot = Some((slugs, leaked));
    Some(leaked)
}
