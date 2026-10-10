use super::text::failure;
use anyhow::Result;
use windows_sys::Win32::Graphics::Gdi::*;

pub(super) struct Font(pub(super) HFONT);

impl Font {
    pub(super) fn new(dpi: u32) -> Result<Self> {
        // SAFETY: LOGFONTW is plain data; zero is a valid default for its fields.
        let mut description: LOGFONTW = unsafe { std::mem::zeroed() };
        description.lfHeight = -((16 * dpi / 96) as i32);
        description.lfWeight = FW_NORMAL as i32;
        description.lfCharSet = DEFAULT_CHARSET;
        for (slot, unit) in description
            .lfFaceName
            .iter_mut()
            .zip("Segoe UI".encode_utf16())
        {
            *slot = unit;
        }
        // SAFETY: GDI copies the initialized descriptor during the call.
        let handle = unsafe { CreateFontIndirectW(&description) };
        if handle.is_null() {
            return Err(failure("Cannot create UI font"));
        }
        Ok(Self(handle))
    }
}

impl Drop for Font {
    fn drop(&mut self) {
        // SAFETY: this guard owns the font; controls stop using it before drop.
        unsafe {
            DeleteObject(self.0);
        }
    }
}
