//! Convenience constructors for browser pages.

use crate::browser_agent::navigation_state::{PageLoadState, PageNavigation};
use crate::browser_agent::page::BrowserPage;
use crate::browser_session::BrowserSession;

impl BrowserPage {
    /// Wrap an existing browser session.
    pub fn new(session: BrowserSession) -> Self {
        let navigation = PageNavigation::new(0, session.url.clone(), "new", PageLoadState::Load);
        let history =
            crate::browser_agent::navigation::PageHistory::new(session.clone(), navigation.id);
        Self {
            session,
            viewport_width: 80,
            viewport_height: 24,
            device_scale: super::viewport::DeviceScale::default(),
            media: crate::browser_agent::exports::media::MediaEmulation::default(),
            wait_options: crate::browser_agent::wait_options::WaitOptions::default(),
            resource_limits: crate::browser_agent::limits::BrowserResourceLimits::default(),
            security_policy: crate::browser_agent::exports::SecurityPolicy::default(),
            permissions: crate::browser_agent::permissions::PermissionStore::default(),
            geolocation: crate::browser_agent::permissions::GeolocationEmulation::default(),
            download_records: Vec::new(),
            resources: super::resources::ResourceRegistry::default(),
            network_routes: crate::browser_agent::network::shared_route_table(),
            action_trace: crate::browser_agent::trace::PageTrace::default(),
            pointer_capture: None,
            frame_windows: crate::browser_agent::frames::FrameWindowState::default(),
            runtime: None,
            navigation,
            history,
            context_state: None,
            event_log: Vec::new(),
        }
    }

    /// Create a page loaded with in-memory HTML.
    pub fn from_html(url: impl Into<String>, html: impl Into<String>) -> Self {
        let mut page = Self::new(BrowserSession::new());
        page.goto_html(url, html);
        page
    }
}
