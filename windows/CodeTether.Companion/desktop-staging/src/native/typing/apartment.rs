use crate::TypingError;
use std::{marker::PhantomData, rc::Rc};
use windows::Win32::System::Com::{COINIT_MULTITHREADED, CoInitializeEx, CoUninitialize};

pub(super) struct Apartment(PhantomData<Rc<()>>);
impl Apartment {
    pub(super) fn enter() -> Result<Self, TypingError> {
        // SAFETY: the dedicated input worker initializes and uninitializes on one thread.
        unsafe { CoInitializeEx(None, COINIT_MULTITHREADED) }
            .ok()
            .map_err(|_| TypingError::Unavailable)?;
        Ok(Self(PhantomData))
    }
}
impl Drop for Apartment {
    fn drop(&mut self) {
        // SAFETY: all UIA interfaces are dropped before this apartment guard.
        unsafe {
            CoUninitialize();
        }
    }
}
