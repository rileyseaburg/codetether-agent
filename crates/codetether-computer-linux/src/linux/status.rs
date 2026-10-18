//! Report implementation support separately from desktop capture readiness.

use super::{backend, session::Session};
use codetether_computer_contract::response;

pub(super) fn result(session: Session) -> codetether_tool_core::ToolResult {
    let backends: Vec<_> = backend::available(session)
        .iter()
        .map(|(backend, _)| backend.name())
        .collect();
    response::success_result(serde_json::json!({
        "supported": true, "platform": "linux", "session_type": session.name(),
        "supported_actions": ["status", "snapshot"],
        "graphical_session": session != Session::Headless,
        "screen_capture_ready": session != Session::Headless && !backends.is_empty(),
        "installed_backends": backends,
        "permissions": "Current logged-in graphical session and compositor capture permission; an installed utility does not guarantee access.",
        "setup": "GNOME: gnome-screenshot; KDE: spectacle; wlroots Wayland: grim; X11: scrot, maim, or ImageMagick import. Linux window targeting, input, and OCR are not supported."
    }))
}
