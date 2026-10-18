//! Mouse event routing and view-aware scroll policy.
//!
//! [handle_mouse_event] gives help, approval and other overlays priority before
//! scrolling the current view. Clicks try editor navigation before file opening.
//! [Host] supplies native click/approval/audit operations and the existing
//! navigation context/action interface.
use crossterm::event::KeyCode;
use std::{future::Future, path::Path};
mod overlay_scroll;
mod run;
mod scroll_down;
mod scroll_up;
pub use run::{handle_mouse_event, scroll_mouse_down, scroll_mouse_up};
/// Native mouse operations beyond ordinary navigation actions.
/// # Implementors
/// Frontends reuse their navigation Host and bind clicks, approvals and audit lists.
pub trait Host: crate::Host {
    /// Give the approval overlay first refusal on a scroll event.
    fn approval_wheel(&mut self, key: KeyCode, amount: usize) -> bool;
    /// Navigate inside the editor, returning whether the click was consumed.
    fn editor_click(&mut self, cwd: &Path, column: u16, row: u16) -> impl Future<Output = bool>;
    /// Open the path under the mouse after the editor declines the click.
    fn click_open_path(&mut self, cwd: &Path, column: u16, row: u16);
    /// Select the preceding audit record.
    fn audit_previous(&mut self);
    /// Select the next audit record.
    fn audit_next(&mut self);
}
