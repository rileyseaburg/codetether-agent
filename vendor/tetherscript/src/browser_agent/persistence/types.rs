//! Snapshot data types for browser-agent persistence.

use crate::browser_agent::permissions::{GeolocationEmulation, PermissionGrant};
use crate::browser_agent::{CacheRecord, ServiceWorkerRegistration};
use crate::browser_agent::{DownloadRecord, IndexedDbRecord, MediaEmulation, Viewport};
use crate::browser_session::Cookie;

/// Snapshot of one origin-scoped storage bucket.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::StorageOriginSnapshot;
///
/// let snapshot = StorageOriginSnapshot {
///     origin: "https://example.test".into(),
///     entries: vec![("token".into(), "abc".into())],
/// };
/// assert_eq!(snapshot.entries.len(), 1);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct StorageOriginSnapshot {
    /// Origin key such as `https://example.test`.
    pub origin: String,
    /// Deterministically sorted key/value entries for the origin.
    pub entries: Vec<(String, String)>,
}

/// Shared native browser storage snapshot.
///
/// This excludes per-page `sessionStorage`, which remains tab-scoped.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct BrowserStorageState {
    /// Cookie jar shared by pages in one browser context.
    pub cookies: Vec<Cookie>,
    /// Origin-scoped localStorage buckets.
    pub local_storage: Vec<StorageOriginSnapshot>,
    /// Shared context IndexedDB-like records.
    pub indexed_db: Vec<IndexedDbRecord>,
    /// Shared service-worker registrations.
    pub service_workers: Vec<ServiceWorkerRegistration>,
    /// Shared CacheStorage records.
    pub caches: Vec<CacheRecord>,
}

/// Native snapshot of one [`BrowserPage`](crate::browser_agent::BrowserPage).
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::BrowserPage;
///
/// let snapshot = BrowserPage::from_html("mem://page", "<main>Page</main>").snapshot_state();
/// assert_eq!(snapshot.html, "<main>Page</main>");
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct BrowserPageSnapshot {
    /// Current page URL.
    pub url: String,
    /// Serialized current DOM HTML.
    pub html: String,
    /// Cookie jar visible to the page/session.
    pub cookies: Vec<Cookie>,
    /// Origin-scoped localStorage buckets.
    pub local_storage: Vec<StorageOriginSnapshot>,
    /// Origin-scoped sessionStorage buckets.
    pub session_storage: Vec<StorageOriginSnapshot>,
    /// Viewport and device metadata.
    pub viewport: Viewport,
    /// Media emulation metadata.
    pub media: MediaEmulation,
    /// Origin-scoped page permission decisions.
    pub permissions: Vec<PermissionGrant>,
    /// Page geolocation emulation metadata.
    pub geolocation: GeolocationEmulation,
    /// Deterministic page download records.
    pub downloads: Vec<DownloadRecord>,
}

/// Native snapshot of a [`BrowserContext`](crate::browser_agent::BrowserContext).
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::BrowserContext;
///
/// let snapshot = BrowserContext::new().snapshot_state();
/// assert!(snapshot.pages.is_empty());
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct BrowserContextSnapshot {
    /// Shared context cookie jar.
    pub cookies: Vec<Cookie>,
    /// Shared context localStorage buckets.
    pub local_storage: Vec<StorageOriginSnapshot>,
    /// Shared context IndexedDB-like records.
    pub indexed_db: Vec<IndexedDbRecord>,
    /// Shared service-worker registrations.
    pub service_workers: Vec<ServiceWorkerRegistration>,
    /// Shared CacheStorage records.
    pub caches: Vec<CacheRecord>,
    /// Context default permission decisions.
    pub permissions: Vec<PermissionGrant>,
    /// Context default geolocation emulation metadata.
    pub geolocation: GeolocationEmulation,
    /// Page snapshots in context order.
    pub pages: Vec<BrowserPageSnapshot>,
}
