//! Page wrapper around a deterministic browser session.

#[path = "cssom/mod.rs"]
pub mod cssom;
#[path = "diagnostics/mod.rs"]
pub mod diagnostics;
#[path = "page_bounds.rs"]
mod page_bounds;
#[path = "page_factory.rs"]
mod page_factory;
#[path = "page_traits.rs"]
mod page_traits;
#[path = "resources/mod.rs"]
pub(crate) mod resources;
#[path = "selection.rs"]
mod selection;
#[cfg(test)]
#[path = "selection_tests.rs"]
mod selection_tests;
#[path = "viewport.rs"]
pub(crate) mod viewport;

use crate::browser_agent::context::context_state::SharedContextState;
use crate::browser_agent::navigation_state::PageNavigation;
use crate::browser_agent::wait_options::WaitOptions;
use crate::browser_js::BrowserJsRuntime;
use crate::browser_session::BrowserSession;

/// A single agent-controlled page.
pub struct BrowserPage {
    pub session: BrowserSession,
    pub viewport_width: i64,
    pub viewport_height: i64,
    pub device_scale: viewport::DeviceScale,
    pub media: super::exports::media::MediaEmulation,
    pub wait_options: WaitOptions,
    pub resource_limits: super::limits::BrowserResourceLimits,
    pub(crate) security_policy: super::exports::SecurityPolicy,
    pub(crate) permissions: super::permissions::PermissionStore,
    pub(crate) geolocation: super::permissions::GeolocationEmulation,
    pub(crate) download_records: Vec<super::downloads::DownloadRecord>,
    pub(crate) resources: resources::ResourceRegistry,
    pub(crate) network_routes: super::network::SharedRouteTable,
    pub(crate) action_trace: super::trace::PageTrace,
    pub(crate) pointer_capture: Option<Vec<usize>>,
    pub(crate) frame_windows: super::frames::FrameWindowState,
    pub(crate) runtime: Option<BrowserJsRuntime>,
    pub(crate) navigation: PageNavigation,
    pub(crate) history: super::navigation::PageHistory,
    pub(crate) context_state: Option<SharedContextState>,
    pub(crate) event_log: Vec<super::events::PageEventSummary>,
}
