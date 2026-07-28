//! Public re-exports for the browser agent module.

#[path = "accessibility/mod.rs"]
pub(crate) mod accessibility;
#[path = "canvas.rs"]
pub(crate) mod canvas;
#[path = "files.rs"]
mod files;
#[path = "media.rs"]
pub(crate) mod media;
#[path = "realtime/mod.rs"]
pub(crate) mod realtime;
#[path = "security/mod.rs"]
pub(crate) mod security;
#[path = "upload.rs"]
mod upload;

pub use super::action::{ActionReport, BoundingBox};
pub use super::action_checks::ActionabilityReport;
pub use super::context::context_state::BrowserContextState;
pub use super::context::indexed_db::{IndexedDbRecord, IndexedDbStore};
pub use super::context::persistence::*;
pub use super::context::service_worker::*;
pub use super::context::BrowserContext;
pub use super::dialog::{DialogDecision, DialogKind, DialogRecord};
pub use super::downloads::{DownloadRecord, DownloadStatus};
pub use super::events::{PageErrorEvent, PageEventKind, PageEventSummary};
pub use super::frames::{BrowserFrame, FrameId, FrameTree};
pub use super::interact::focus::FocusTarget;
pub use super::keyboard::KeyboardKey;
pub use super::limits::{BrowserGuardMetadata, BrowserResourceLimits};
pub use super::locator::{Locator, LocatorKind};
pub use super::navigation::{NavigationKind, NavigationResult, NavigationStatus, PageHistoryEntry};
pub use super::navigation_state::{PageLoadState, PageNavigation};
pub use super::network::{
    NetworkLogEntry, NetworkRoute, RouteAction, RouteFulfillment, RouteId, RoutePattern,
    RouteRequest, RouteRule, RouteTable,
};
pub use super::page::resources::{
    BrowserResource, ImageResourceMetadata, ResourceKind, ResourcePayload,
};
pub use super::page::viewport::{DeviceScale, Viewport};
pub use super::page::BrowserPage;
pub use super::page::{cssom::ComputedStyle, diagnostics::*};
pub use super::screenshot::ElementScreenshot;
pub use super::trace::{ActionSnapshot, ActionTraceEntry, PageTrace};
pub use super::wait_options::WaitOptions;
pub use accessibility::{AccessibilityNode, AccessibilitySnapshot, AccessibilityState};
pub use canvas::{CanvasCommand, CanvasSurface, WebGlCommand, WebGlContextSnapshot};
pub use files::FilePayload;
pub use media::{ColorScheme, ForcedColors, MediaEmulation, ReducedMotion};
pub use realtime::*;
pub use security::{Origin, RequestSecurityMetadata, SandboxFlags, SecurityPolicy};
