//! Visible background sharing state, including while the window is hidden.
use super::{
    text::{failure, wide},
    tray::Tray,
};
use anyhow::Result;
use windows_sys::Win32::UI::Shell::{NIF_TIP, NIM_MODIFY, Shell_NotifyIconW};

impl Tray {
    pub(super) fn sharing(&mut self, active: bool) -> Result<()> {
        let text = if active {
            "CodeTether — sharing ON; iPhone requests enabled"
        } else {
            "CodeTether — sharing OFF / suspended"
        };
        let tip = wide(text);
        if self.data.szTip[..tip.len()] == tip {
            return Ok(());
        }
        let previous = self.data.szTip;
        self.data.szTip.fill(0);
        self.data.szTip[..tip.len()].copy_from_slice(&tip);
        self.data.uFlags = NIF_TIP;
        // SAFETY: this live tray icon and terminated text belong to the UI thread.
        if unsafe { Shell_NotifyIconW(NIM_MODIFY, &self.data) } == 0 {
            self.data.szTip = previous;
            return Err(failure("Cannot update sharing indicator"));
        }
        Ok(())
    }
}
