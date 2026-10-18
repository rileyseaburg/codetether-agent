//! Borrowed approval editor state and a frontend-selected chat transition.
use super::Host;
use crate::ApprovalEditSession;
use codetether_editor::FileBuffer;

/// Approval editor host built from independent frontend fields.
///
/// The callback returns to chat through the frontend's navigation policy.
/// The host owns all other field updates required by approval editing.
///
/// # Examples
/// ```rust
/// use codetether_approval_edit::interactive::{BorrowedHost, Host};
/// let (mut editor, mut session) = (None, None);
/// let (mut vertical, mut horizontal, mut preview) = (4, 2, 3);
/// let (mut waiting, mut redraw) = (false, false);
/// let mut status = String::new();
/// let mut host = BorrowedHost {
///     editor: &mut editor, session: &mut session,
///     vertical_scroll: &mut vertical, horizontal_scroll: &mut horizontal,
///     preview_scroll: &mut preview, waiting: &mut waiting,
///     status: &mut status, needs_redraw: &mut redraw, show_chat: || {},
/// };
/// host.reset_scroll();
/// assert_eq!(*host.vertical_scroll, 0);
/// ```
pub struct BorrowedHost<'a, F> {
    /// Active proposal document.
    pub editor: &'a mut Option<FileBuffer>,
    /// Active proposed patch editing session.
    pub session: &'a mut Option<ApprovalEditSession>,
    /// First visible editor line.
    pub vertical_scroll: &'a mut usize,
    /// First visible editor column.
    pub horizontal_scroll: &'a mut usize,
    /// First visible approval preview line.
    pub preview_scroll: &'a mut u16,
    /// Whether an approval remains active.
    pub waiting: &'a mut bool,
    /// Frontend status text.
    pub status: &'a mut String,
    /// Frontend redraw request.
    pub needs_redraw: &'a mut bool,
    /// Return to chat, preserving frontend navigation policy.
    pub show_chat: F,
}

impl<F: FnMut()> Host for BorrowedHost<'_, F> {
    fn editor(&mut self) -> &mut Option<FileBuffer> {
        self.editor
    }
    fn session(&mut self) -> &mut Option<ApprovalEditSession> {
        self.session
    }
    fn reset_scroll(&mut self) {
        *self.vertical_scroll = 0;
        *self.horizontal_scroll = 0;
    }
    fn show_chat(&mut self) {
        (self.show_chat)();
    }
    fn reset_preview_scroll(&mut self) {
        *self.preview_scroll = 0;
    }
    fn set_waiting(&mut self, waiting: bool) {
        *self.waiting = waiting;
    }
    fn set_status(&mut self, status: String) {
        *self.status = status;
    }
    fn redraw(&mut self) {
        *self.needs_redraw = true;
    }
}
