//! Fail closed instead of typing into whichever application stole foreground focus.

use crate::tool::computer_use::input::ComputerUseInput;
use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::{
    GA_ROOT, GetAncestor, GetForegroundWindow, IsWindow,
}};

pub(super) fn require(input: &ComputerUseInput) -> anyhow::Result<i64> {
    let id = input.hwnd.filter(|id| *id > 0)
        .ok_or_else(|| anyhow::anyhow!("Physical keyboard input requires an explicit hwnd; bring that window to front first"))?;
    let target = HWND(id as isize as *mut core::ffi::c_void);
    anyhow::ensure!(unsafe { IsWindow(Some(target)) }.as_bool(), "Keyboard target is no longer a live window");
    let foreground = unsafe { GetForegroundWindow() };
    let root = unsafe { GetAncestor(target, GA_ROOT) };
    let foreground_root = unsafe { GetAncestor(foreground, GA_ROOT) };
    anyhow::ensure!(!root.is_invalid() && root == foreground_root,
        "Keyboard target is not foreground; refusing input that could affect CodeTether or another application");
    tracing::info!(hwnd = id, "Physical keyboard target verified");
    Ok(id)
}