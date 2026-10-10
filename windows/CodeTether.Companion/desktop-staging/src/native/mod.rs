//! Windows eligibility snapshots, display discovery, and bounded GDI capture.
mod callback;
mod typing;
pub(crate) use typing::type_focused;
mod capture;
mod desktop;
mod discovery;
mod dpi;
mod monitor_info;
mod object;
mod session;
mod station;
pub(crate) use capture::capture;

use crate::{Error, Monitor};

pub(crate) fn check_available() -> Result<(), Error> {
    session::check()?;
    station::check()?;
    desktop::check()
}

pub(crate) fn monitors() -> Result<Vec<Monitor>, Error> {
    check_available()?;
    let dpi = dpi::Guard::enter()?;
    // Restore explicitly even when discovery fails, so restoration failures
    // are reported rather than silently discarded by the fallback Drop guard.
    let monitors = stable_snapshot();
    dpi.restore()?;
    check_available()?;
    monitors
}

fn stable_snapshot() -> Result<Vec<Monitor>, Error> {
    let first = discovery::enumerate()?;
    check_available()?;
    let second = discovery::enumerate()?;
    if first != second {
        return Err(Error::Selection);
    }
    Ok(second)
}
