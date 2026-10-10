use crate::Error;
use std::mem::size_of_val;
use windows_sys::Win32::{Foundation::HANDLE, System::StationsAndDesktops::*};

/// `USEROBJECTFLAGS.dwFlags` visibility bit (`WSF_VISIBLE` in winuser.h);
/// not exported by windows-sys 0.61.
const WSF_VISIBLE: u32 = 0x0001;

pub(super) fn name_is(handle: HANDLE, expected: &str) -> Result<(), Error> {
    if handle.is_null() {
        return Err(Error::Unavailable);
    }
    let mut name = [0u16; 256];
    let mut bytes = 0;
    // SAFETY: name is a writable bounded UTF-16 buffer; handle is borrowed.
    let ok = unsafe {
        GetUserObjectInformationW(
            handle,
            UOI_NAME,
            name.as_mut_ptr().cast(),
            size_of_val(&name) as u32,
            &mut bytes,
        )
    };
    if ok == 0 || bytes < 2 || bytes as usize > size_of_val(&name) || bytes % 2 != 0 {
        return Err(Error::Unavailable);
    }
    let units = &name[..bytes as usize / 2];
    let Some((&0, text)) = units.split_last() else {
        return Err(Error::Unavailable);
    };
    let text = String::from_utf16(text).map_err(|_| Error::Unavailable)?;
    if text.eq_ignore_ascii_case(expected) {
        Ok(())
    } else {
        Err(Error::Unavailable)
    }
}

pub(super) fn visible(handle: HANDLE) -> Result<(), Error> {
    let mut flags = USEROBJECTFLAGS::default();
    let mut bytes = 0;
    // SAFETY: flags is valid output storage of exactly the supplied size.
    let ok = unsafe {
        GetUserObjectInformationW(
            handle,
            UOI_FLAGS,
            std::ptr::from_mut(&mut flags).cast(),
            size_of_val(&flags) as u32,
            &mut bytes,
        )
    };
    if ok != 0 && bytes as usize == size_of_val(&flags) && flags.dwFlags & WSF_VISIBLE != 0 {
        Ok(())
    } else {
        Err(Error::Unavailable)
    }
}
