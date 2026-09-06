//! Post only to the validated HWND; never synthesize hardware input.
use super::super::types::Event;
use super::Target;
use anyhow::{Context, Result, ensure};
use windows::Win32::{
    Foundation::{LPARAM, WPARAM},
    UI::WindowsAndMessaging::PostMessageW,
};
impl Target {
    pub fn post(self, event: &Event) -> Result<()> {
        if event.delay_ms != 0 {
            std::thread::sleep(std::time::Duration::from_millis(event.delay_ms));
        }
        ensure!(
            Self::open(self.hwnd)? == self,
            "Target HWND identity changed before posting"
        );
        unsafe {
            PostMessageW(Some(self.handle()), event.message, WPARAM(event.wparam), LPARAM(event.lparam))
        }.with_context(|| format!("PostMessageW failed for HWND {} message {:#x}; UIPI or queue limits may deny posting; no physical fallback", self.hwnd, event.message))
    }
}
