//! Serialization helpers for service-worker bridge seed data.

use crate::browser_agent::context::service_worker::{CacheRecord, ServiceWorkerRegistration};
use crate::browser_agent::keyboard_escape::quote;

pub(super) fn registrations(items: &[ServiceWorkerRegistration]) -> String {
    let body = items
        .iter()
        .map(|item| {
            format!(
                "{{origin:{},scope:{},scriptURL:{},state:{}}}",
                quote(&item.origin),
                quote(&item.scope),
                quote(&item.script_url),
                quote(&format!("{:?}", item.state).to_ascii_lowercase())
            )
        })
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

pub(super) fn caches(items: &[CacheRecord], origin: &str) -> String {
    let body = items
        .iter()
        .map(|item| cache(item, origin))
        .collect::<Vec<_>>()
        .join(",");
    format!("[{body}]")
}

fn cache(item: &CacheRecord, origin: &str) -> String {
    let path = item
        .request_url
        .strip_prefix(origin)
        .unwrap_or(&item.request_url);
    let name = path.strip_prefix('/').unwrap_or(path);
    format!(
        "{{origin:{},cacheName:{},requestURL:{},requestPath:{},requestName:{},status:{},body:{}}}",
        quote(&item.origin),
        quote(&item.cache_name),
        quote(&item.request_url),
        quote(path),
        quote(name),
        item.response.status,
        quote(&item.response.body)
    )
}
