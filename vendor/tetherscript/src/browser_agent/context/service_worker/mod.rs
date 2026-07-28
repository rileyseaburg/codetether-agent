//! Deterministic service-worker and CacheStorage metadata.

mod cache_access;
mod cache_api;
mod cache_context_api;
mod cache_delete;
mod cache_list;
mod cache_record;
mod cache_store;
mod context_api;
mod fetch_event;
mod fetch_log;
mod handler;
mod intercept;
mod js_bridge;
mod origin;
mod page_api;
mod page_cache_api;
mod page_cache_more;
mod page_state;
mod registration;
mod registration_api;
mod registry;
mod registry_read;
mod replace;

#[cfg(test)]
mod tests_cache;
#[cfg(test)]
mod tests_context;
#[cfg(test)]
mod tests_fetch;
#[cfg(test)]
mod tests_js_bridge;
#[cfg(test)]
mod tests_persistence;

pub use cache_record::{CacheRecord, CacheResponse};
pub(crate) use cache_store::CacheStore;
pub use fetch_event::ServiceWorkerFetchEvent;
pub(crate) use handler::service_worker_route_handler;
pub use registration::{ServiceWorkerRegistration, ServiceWorkerState};
pub use registry::ServiceWorkerStore;
