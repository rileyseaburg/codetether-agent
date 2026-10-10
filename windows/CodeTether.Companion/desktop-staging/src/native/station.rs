use super::object;
use crate::Error;
use windows_sys::Win32::System::StationsAndDesktops::GetProcessWindowStation;

pub(super) fn check() -> Result<(), Error> {
    // SAFETY: returns a borrowed process station, which must not be closed here.
    let station = unsafe { GetProcessWindowStation() };
    object::name_is(station, "WinSta0")?;
    object::visible(station)
}
