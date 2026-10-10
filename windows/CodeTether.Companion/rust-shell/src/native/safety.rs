//! Record safety events even during reentrant Windows control callbacks.
use super::{context::Context, ids, session_events};
use std::sync::atomic::Ordering;
use windows_sys::Win32::UI::WindowsAndMessaging::*;

pub(super) fn record(context: &Context, message: u32, word: usize) {
    if message == WM_COMMAND {
        let id = (word & 0xffff) as u16;
        let notification = (word >> 16) as u32;
        let actionable = if id == ids::PICKER {
            notification == CBN_SELCHANGE
        } else {
            notification == BN_CLICKED
        };
        if !actionable {
            return;
        }
        let queued_stop = context
            .command
            .get()
            .is_some_and(|v| matches!((v & 0xffff) as u16, ids::STOP | ids::EXIT));
        if !queued_stop {
            context.command.set(Some(word));
        }
        if matches!(
            id,
            ids::PAUSE | ids::STOP | ids::EXIT | ids::PAIR | ids::PICKER | ids::REFRESH
        ) {
            context.interrupt.store(true, Ordering::Release);
        }
    }
    if message == WM_WTSSESSION_CHANGE && session_events::invalidates(word) {
        if matches!(
            word as u32,
            WTS_SESSION_LOCK | WTS_SESSION_LOGOFF | WTS_CONSOLE_DISCONNECT | WTS_REMOTE_DISCONNECT
        ) {
            context.session_blocked.set(true);
        }
        if matches!(
            word as u32,
            WTS_SESSION_UNLOCK | WTS_SESSION_LOGON | WTS_CONSOLE_CONNECT | WTS_REMOTE_CONNECT
        ) {
            context.session_blocked.set(false);
        }
        context.invalidate.set(true);
    }
    if matches!(message, WM_DISPLAYCHANGE | ids::RECHECK | WM_DPICHANGED) {
        context.invalidate.set(true);
    }
    if context.invalidate.get() {
        context.interrupt.store(true, Ordering::Release);
    }
}
