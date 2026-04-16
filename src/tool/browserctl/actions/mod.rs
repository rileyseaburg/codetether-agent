//! Dispatcher context + outcome type shared across action groups.

use super::input::BrowserCtlInput;

/// Shared context passed to each per-action-group handler.
pub(super) struct Ctx<'a> {
    pub client: &'a reqwest::Client,
    pub base_url: &'a str,
    pub token: Option<&'a str>,
    pub input: &'a BrowserCtlInput,
}

/// Result tuple produced by every action handler:
/// `(action_name, endpoint_path, http_status, body)`.
pub(super) type Outcome = (&'static str, &'static str, u16, serde_json::Value);

pub(super) mod device;
pub(super) mod dom;
pub(super) mod eval;
pub(super) mod lifecycle;
pub(super) mod nav;
pub(super) mod tabs;
