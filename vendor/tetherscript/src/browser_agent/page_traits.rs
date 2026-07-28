//! Trait implementations for browser pages.

#[path = "page_clone.rs"]
mod page_clone;

use std::fmt;

use crate::browser_agent::page::BrowserPage;

impl fmt::Debug for BrowserPage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BrowserPage")
            .field("session", &self.session)
            .field("viewport", &self.viewport())
            .field("media", &self.media)
            .field("wait_options", &self.wait_options)
            .field("resource_limits", &self.resource_limits)
            .field("security_policy", &self.security_policy)
            .field("permissions", &self.permissions)
            .field("geolocation", &self.geolocation)
            .field("download_records", &self.download_records)
            .field("resources", &self.resources)
            .field("network_routes", &self.network_routes.borrow())
            .field("action_trace", &self.action_trace)
            .field("pointer_capture", &self.pointer_capture)
            .field("frame_windows", &self.frame_windows)
            .field("event_log", &self.event_log)
            .field("navigation", &self.navigation)
            .field("history", &self.history)
            .finish_non_exhaustive()
    }
}

impl PartialEq for BrowserPage {
    fn eq(&self, other: &Self) -> bool {
        self.session == other.session
            && self.viewport() == other.viewport()
            && self.media == other.media
            && self.wait_options == other.wait_options
            && self.resource_limits == other.resource_limits
            && self.security_policy == other.security_policy
            && self.permissions == other.permissions
            && self.geolocation == other.geolocation
            && self.download_records == other.download_records
            && self.resources == other.resources
            && *self.network_routes.borrow() == *other.network_routes.borrow()
            && self.action_trace == other.action_trace
            && self.pointer_capture == other.pointer_capture
            && self.frame_windows == other.frame_windows
            && self.event_log == other.event_log
            && self.navigation == other.navigation
            && self.history == other.history
    }
}
