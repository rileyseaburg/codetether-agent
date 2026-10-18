//! Supported mouse buttons and their Win32 message IDs/masks.

use anyhow::{Result, bail};

#[derive(Clone, Copy)]
pub(super) struct Button {
    pub mask: u16,
    pub down: u32,
    pub up: u32,
    pub double: u32,
}

pub(super) fn parse(name: Option<&str>) -> Result<Button> {
    let (mask, down) = match name.unwrap_or("left") {
        "left" => (1, 0x0201),
        "right" => (2, 0x0204),
        "middle" => (16, 0x0207),
        _ => bail!("Shadow button must be left, right, or middle"),
    };
    Ok(Button {
        mask,
        down,
        up: down + 1,
        double: down + 2,
    })
}
