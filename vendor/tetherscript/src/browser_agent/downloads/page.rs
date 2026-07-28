//! Page download accessors.

use crate::browser_agent::downloads::DownloadRecord;
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Append a deterministic download record to this page.
    pub fn record_download(&mut self, record: DownloadRecord) {
        self.download_records.push(record);
    }

    /// Return page downloads in deterministic insertion order.
    pub fn downloads(&self) -> &[DownloadRecord] {
        &self.download_records
    }

    /// Remove all recorded downloads from this page.
    pub fn clear_downloads(&mut self) {
        self.download_records.clear();
    }
}
