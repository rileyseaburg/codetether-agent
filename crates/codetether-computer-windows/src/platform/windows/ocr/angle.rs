//! Preserve nullable TextAngle without hiding failures from the native getter.

use windows::{Foundation::IReference, core::Result};

pub(super) fn degrees(reference: Result<IReference<f64>>) -> anyhow::Result<Option<f64>> {
    match reference {
        Ok(reference) => Ok(Some(reference.Value()?)),
        // windows-core 0.62 projects a successful null interface to Error::empty(),
        // whose HRESULT is zero; a failed HRESULT must not be mistaken for null.
        Err(error) if error.code().is_ok() => Ok(None),
        Err(error) => Err(error.into()),
    }
}
