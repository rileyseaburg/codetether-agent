//! Balanced WinRT apartment initialization on the calling blocking thread.

use windows::Win32::System::WinRT::{RO_INIT_MULTITHREADED, RoInitialize, RoUninitialize};

pub(super) struct Apartment;

impl Apartment {
    pub(super) fn enter() -> anyhow::Result<Self> {
        // S_OK and S_FALSE both require a matching RoUninitialize.
        unsafe { RoInitialize(RO_INIT_MULTITHREADED)? };
        Ok(Self)
    }
}

impl Drop for Apartment {
    fn drop(&mut self) {
        // The guard and all WinRT objects stay in the same blocking closure.
        unsafe { RoUninitialize() };
    }
}
