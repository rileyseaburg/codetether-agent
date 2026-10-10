#[cfg(windows)]
#[path = "native/mod.rs"]
pub(crate) mod native;
use crate::{Error, Monitor};

/// Check interactive desktop eligibility without capturing or granting consent.
/// Returns `Unavailable` on uncertainty, or `Unsupported` off Windows.
pub fn check_available() -> Result<(), Error> {
    #[cfg(windows)]
    return native::check_available();
    #[cfg(not(windows))]
    Err(Error::Unsupported)
}

/// Enumerate physical-pixel monitor snapshots for explicit local selection.
/// Returns an error on unavailable desktops, invalid geometry or enumeration.
pub fn monitors() -> Result<Vec<Monitor>, Error> {
    #[cfg(windows)]
    return native::monitors();
    #[cfg(not(windows))]
    Err(Error::Unsupported)
}
