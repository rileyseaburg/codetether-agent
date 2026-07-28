//! Browser production-debug report types.

use crate::browser_agent::events::PageErrorEvent;

use super::exception_types::RuntimeException;
use super::har_types::BrowserHarEntry;
use super::mapped_types::SourceMappedPageError;
use super::visual_types::VisualElementEvidence;

/// One-shot production-debug report for a native browser page.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::BrowserPage;
///
/// let page = BrowserPage::from_html("mem://app", "<main></main>");
/// assert_eq!(page.production_debug_report().url, "mem://app");
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserDebugReport {
    /// Current page URL.
    pub url: String,
    /// Browser parity target metadata for this report.
    pub parity: BrowserParityTarget,
    /// Messages captured through `console.error`.
    pub console_errors: Vec<String>,
    /// Runtime errors captured at page action boundaries.
    pub page_errors: Vec<PageErrorEvent>,
    /// Page errors remapped from bundled output to original source files.
    pub mapped_page_errors: Vec<SourceMappedPageError>,
    /// Runtime failures classified for agent triage.
    pub runtime_exceptions: Vec<RuntimeException>,
    /// Failed deterministic network request summaries.
    pub failed_requests: Vec<String>,
    /// HAR-style network entries for production request debugging.
    pub network_har: Vec<BrowserHarEntry>,
    /// Native style and layout evidence for rendered elements.
    pub visual_elements: Vec<VisualElementEvidence>,
    /// Source-map references found in registered script resources.
    pub source_maps: Vec<SourceMapStatus>,
    /// Framework markers detected from the page.
    pub frameworks: Vec<String>,
    /// React-specific production diagnostics.
    pub react: ReactDebugSummary,
}

/// Native browser parity target metadata.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::BrowserParityTarget;
///
/// let parity = BrowserParityTarget { target: "full native browser parity".into(), native_engine: true };
/// assert!(parity.native_engine);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BrowserParityTarget {
    /// Human-readable parity target name.
    pub target: String,
    /// Whether this report came from the native in-tree browser path.
    pub native_engine: bool,
}

/// Source-map registration status for one bundled script.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::SourceMapStatus;
///
/// let status = SourceMapStatus { script_url: "/app.js".into(), source_map_url: "/app.js.map".into(), registered: false };
/// assert!(!status.registered);
/// ```
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct SourceMapStatus {
    /// Script resource that referenced the map.
    pub script_url: String,
    /// Resolved source-map URL.
    pub source_map_url: String,
    /// Whether a matching source-map resource was registered.
    pub registered: bool,
}

/// React-oriented diagnostics collected from native page state.
///
/// # Examples
///
/// ```rust
/// use tetherscript::browser_agent::ReactDebugSummary;
///
/// let summary = ReactDebugSummary::default();
/// assert!(!summary.detected);
/// ```
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ReactDebugSummary {
    /// Whether React markers were found.
    pub detected: bool,
    /// Selectors for detected React roots.
    pub roots: Vec<String>,
    /// Console messages that look like React hydration failures.
    pub hydration_warnings: Vec<String>,
}
