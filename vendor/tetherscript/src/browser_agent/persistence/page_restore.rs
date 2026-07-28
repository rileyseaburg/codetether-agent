//! Page snapshot restore implementation.

use super::storage::restore_storage;
use super::types::BrowserPageSnapshot;
use crate::browser_agent::navigation_state::PageLoadState;
use crate::browser_agent::page::viewport::DeviceScale;
use crate::browser_agent::BrowserPage;
use crate::browser_session::BrowserSession;

pub(super) fn restore(page: &mut BrowserPage, snapshot: BrowserPageSnapshot) -> Result<(), String> {
    validate_viewport(snapshot.viewport.width, snapshot.viewport.height)?;
    let device = DeviceScale::new(
        snapshot.viewport.device_scale_factor,
        snapshot.viewport.is_mobile,
    )?;
    let context_state = page.context_state.clone();
    page.session = BrowserSession::new();
    page.session.goto_html(snapshot.url, snapshot.html);
    page.session.cookies = snapshot.cookies;
    page.session.local_storage = restore_storage(&snapshot.local_storage);
    page.session.session_storage = restore_storage(&snapshot.session_storage);
    page.viewport_width = snapshot.viewport.width;
    page.viewport_height = snapshot.viewport.height;
    page.device_scale = device;
    page.media = snapshot.media;
    page.permissions.replace_all(snapshot.permissions);
    page.geolocation = snapshot.geolocation;
    page.download_records = snapshot.downloads;
    page.runtime = None;
    page.navigation = page.navigation.next(
        page.session.url.clone(),
        "restore_state",
        PageLoadState::Load,
    );
    page.context_state = context_state;
    page.sync_context_state_from_session();
    Ok(())
}

fn validate_viewport(width: i64, height: i64) -> Result<(), String> {
    if width <= 0 || height <= 0 {
        Err("snapshot viewport width and height must be positive".into())
    } else {
        Ok(())
    }
}
