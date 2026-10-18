//! Read outer/client origins in the caller's Win32 DPI coordinate context.
use super::super::types::{Geometry, Point};
use anyhow::{Context, Result};
use windows::Win32::{
    Foundation::{HWND, POINT, RECT},
    Graphics::Gdi::ClientToScreen,
    UI::WindowsAndMessaging::GetWindowRect,
};

pub(super) fn read(hwnd: HWND) -> Result<Geometry> {
    let mut rect = RECT::default();
    let mut client = POINT::default();
    unsafe {
        GetWindowRect(hwnd, &mut rect).context("GetWindowRect failed")?;
        ClientToScreen(hwnd, &mut client)
            .ok()
            .context("ClientToScreen failed")?;
    }
    Ok(Geometry {
        outer: Point {
            x: rect.left,
            y: rect.top,
        },
        client: Point {
            x: client.x,
            y: client.y,
        },
    })
}
