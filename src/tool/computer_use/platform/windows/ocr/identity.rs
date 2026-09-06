//! Supported desktop OCR requires package identity; the installer supplies it.

use windows::{
    Win32::{Foundation::{APPMODEL_ERROR_NO_PACKAGE, ERROR_INSUFFICIENT_BUFFER, ERROR_SUCCESS},
        Storage::Packaging::Appx::GetCurrentPackageFullName},
    core::PWSTR,
};

pub(super) const SETUP_HINT: &str = "Run the CodeTether Windows installer to provision native OCR and its package identity automatically, then launch the installed codetether app alias. No Python or Tesseract setup is needed.";

pub(super) fn package_name() -> anyhow::Result<Option<String>> {
    let mut length = 0;
    let status = unsafe { GetCurrentPackageFullName(&mut length, None) };
    if status == APPMODEL_ERROR_NO_PACKAGE { return Ok(None); }
    anyhow::ensure!(status == ERROR_INSUFFICIENT_BUFFER && length > 0,
        "Cannot query Windows package identity: {}", status.0);
    let mut name = vec![0u16; length as usize];
    let status = unsafe { GetCurrentPackageFullName(&mut length, Some(PWSTR(name.as_mut_ptr()))) };
    anyhow::ensure!(status == ERROR_SUCCESS, "Cannot read Windows package identity: {}", status.0);
    let end = name.iter().position(|unit| *unit == 0).unwrap_or(name.len());
    Ok(Some(String::from_utf16(&name[..end])?))
}

pub(super) fn require() -> anyhow::Result<()> {
    anyhow::ensure!(package_name()?.is_some(), "Windows OCR requires package identity. {SETUP_HINT}");
    Ok(())
}