//! Page snapshot and restore APIs.

use super::storage::snapshot_storage;
use super::types::BrowserPageSnapshot;
use crate::browser_agent::BrowserPage;

impl BrowserPage {
    /// Capture serializable page state without cloning the JavaScript heap.
    ///
    /// # Returns
    ///
    /// A native Rust snapshot containing URL, HTML, storage, viewport, media,
    /// and deterministic download metadata. Event listeners, closures, timers,
    /// and other JavaScript heap values are intentionally not included.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let page = BrowserPage::from_html("mem://snapshot", "<main>Saved</main>");
    /// let snapshot = page.snapshot_state();
    /// assert_eq!(snapshot.url, "mem://snapshot");
    /// ```
    pub fn snapshot_state(&self) -> BrowserPageSnapshot {
        let (cookies, local_storage) = super::page_shared::shared_storage_view(self);
        BrowserPageSnapshot {
            url: self.session.url.clone(),
            html: self.session.html.clone(),
            cookies,
            local_storage: snapshot_storage(&local_storage),
            session_storage: snapshot_storage(&self.session.session_storage),
            viewport: self.viewport(),
            media: self.media,
            permissions: self.permissions.grants(),
            geolocation: self.geolocation.clone(),
            downloads: self.download_records.clone(),
        }
    }

    /// Restore page state and start a fresh JavaScript runtime.
    ///
    /// # Arguments
    ///
    /// * `snapshot` - Native page snapshot previously returned by
    ///   [`BrowserPage::snapshot_state`].
    ///
    /// # Errors
    ///
    /// Returns `Err` when the snapshot contains invalid viewport or device
    /// scale metadata.
    ///
    /// # Examples
    ///
    /// ```
    /// use tetherscript::browser_agent::BrowserPage;
    ///
    /// let page = BrowserPage::from_html("mem://restore", "<main>Saved</main>");
    /// let snapshot = page.snapshot_state();
    /// let mut restored = BrowserPage::new(Default::default());
    /// restored.restore_state(snapshot).unwrap();
    /// assert_eq!(restored.session.url, "mem://restore");
    /// ```
    pub fn restore_state(&mut self, snapshot: BrowserPageSnapshot) -> Result<(), String> {
        super::page_restore::restore(self, snapshot)
    }
}
