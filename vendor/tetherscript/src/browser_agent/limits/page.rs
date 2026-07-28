//! Browser page limit accessors and guards.

use crate::browser_agent::limits::checks::{dom_bytes, enforce_dom_bytes};
use crate::browser_agent::limits::{BrowserGuardMetadata, BrowserResourceLimits};
use crate::browser_agent::page::BrowserPage;

impl BrowserPage {
    /// Return the current resource limits for this page.
    pub fn resource_limits(&self) -> BrowserResourceLimits {
        self.resource_limits
    }

    /// Replace this page's resource limits.
    pub fn set_resource_limits(&mut self, limits: BrowserResourceLimits) {
        self.wait_options.timeout_ticks = limits.retry_ticks();
        self.resource_limits = limits;
    }

    /// Return current guard metadata for diagnostics and traces.
    pub fn guard_metadata(&self) -> BrowserGuardMetadata {
        let limits = self.resource_limits;
        BrowserGuardMetadata {
            dom_bytes: dom_bytes(&self.session.html),
            trace_entries: self.action_trace.entries().len(),
            max_dom_bytes: limits.max_dom_bytes,
            max_trace_entries: limits.max_trace_entries,
            max_action_attempts: limits.max_action_attempts,
            max_action_ticks: limits.max_action_ticks,
            max_raster_pixels: limits.max_raster_pixels,
        }
    }

    pub(crate) fn enforce_resource_limits(&self, operation: &str) -> Result<(), String> {
        enforce_dom_bytes(
            operation,
            &self.session.html,
            self.resource_limits.max_dom_bytes,
        )
    }
}
