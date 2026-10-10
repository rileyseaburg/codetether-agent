use crate::{Error, Monitor, monitors};

/// Recheck desktop eligibility and require the exact locally selected snapshot.
///
/// # Arguments
/// * `selected` — A previous local choice returned by [`monitors`].
/// # Returns
/// The matching current monitor; this does not grant capture consent.
/// # Errors
/// Returns a desktop/discovery error or [`Error::Selection`] if bounds or
/// identity changed. A reconnect with identical metadata cannot be detected
/// here: the host must invalidate consent on display-change events.
/// # Examples
/// ```no_run
/// # fn main() -> Result<(), codetether_companion_desktop::Error> {
/// use codetether_companion_desktop::{monitors, validate_selection};
/// if let Some(selected) = monitors()?.first() {
///     assert_eq!(&validate_selection(selected)?, selected);
/// }
/// # Ok(()) }
/// ```
pub fn validate_selection(selected: &Monitor) -> Result<Monitor, Error> {
    select(selected, monitors()?)
}

pub(crate) fn select(selected: &Monitor, current: Vec<Monitor>) -> Result<Monitor, Error> {
    current
        .into_iter()
        .find(|item| item == selected)
        .ok_or(Error::Selection)
}
