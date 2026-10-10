//! Top-down BGRA DIB ownership and zeroing, including early-return paths.
use super::gdi::Context;
use crate::{Bounds, Error};
use std::{ffi::c_void, ptr};
use windows_sys::Win32::Graphics::Gdi::*;
use zeroize::Zeroize;

pub(super) struct Surface<'a> {
    context: &'a Context,
    bitmap: HBITMAP,
    previous: HGDIOBJ,
    pixels: *mut c_void,
    len: usize,
}
impl<'a> Surface<'a> {
    pub(super) fn new(context: &'a Context, bounds: Bounds) -> Result<Self, Error> {
        // SAFETY: BITMAPINFO contains only integer fields and a color array.
        let mut info: BITMAPINFO = unsafe { std::mem::zeroed() };
        info.bmiHeader.biSize = size_of::<BITMAPINFOHEADER>() as u32;
        info.bmiHeader.biWidth = bounds.width() as i32;
        info.bmiHeader.biHeight = -(bounds.height() as i32);
        info.bmiHeader.biPlanes = 1;
        info.bmiHeader.biBitCount = 32;
        info.bmiHeader.biCompression = BI_RGB;
        let mut pixels = ptr::null_mut();
        // SAFETY: initialized 32-bit top-down geometry is bounded by Bounds.
        let bitmap = unsafe {
            CreateDIBSection(
                context.target,
                &info,
                DIB_RGB_COLORS,
                &mut pixels,
                ptr::null_mut(),
                0,
            )
        };
        let mut surface = Self {
            context,
            bitmap,
            previous: ptr::null_mut(),
            pixels,
            len: bounds.width() as usize * bounds.height() as usize * 4,
        };
        if bitmap.is_null() || pixels.is_null() {
            return Err(Error::Capture);
        }
        // SAFETY: bitmap and dc are live; the original selection is restored.
        surface.previous = unsafe { SelectObject(context.target, bitmap) };
        if surface.previous.is_null() || surface.previous as isize == -1 {
            return Err(Error::Capture);
        }
        Ok(surface)
    }
    pub(super) fn bytes(&mut self) -> &[u8] {
        // SAFETY: DIB owns len bytes; caller flushes GDI first; borrow cannot escape.
        unsafe { std::slice::from_raw_parts(self.pixels.cast::<u8>(), self.len) }
    }
}
impl Drop for Surface<'_> {
    fn drop(&mut self) {
        // SAFETY: flush queued GDI writes before clearing and releasing owned memory.
        unsafe {
            GdiFlush();
            if !self.bitmap.is_null() && !self.pixels.is_null() {
                std::slice::from_raw_parts_mut(self.pixels.cast::<u8>(), self.len).zeroize();
            }
            if !self.previous.is_null() && self.previous as isize != -1 {
                SelectObject(self.context.target, self.previous);
            }
            if !self.bitmap.is_null() {
                DeleteObject(self.bitmap);
            }
        }
    }
}
