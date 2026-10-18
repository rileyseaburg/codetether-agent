//! Returns an approval editor session to the chat view.

use super::Host;
use codetether_approval_queue as approval_queue;

pub(super) fn close(host: &mut impl Host, status: String) {
    *host.session() = None;
    *host.editor() = None;
    host.reset_scroll();
    host.show_chat();
    host.reset_preview_scroll();
    host.set_waiting(approval_queue::active().is_some());
    host.set_status(status);
}
