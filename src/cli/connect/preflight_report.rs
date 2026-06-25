//! Pure validation of the remote binary preflight probe report.
//!
//! Extracted from [`preflight`](super::preflight) so the classification logic
//! can be unit-tested without SSH.

/// Classify the probe report and return `Ok(())` when the binary is a valid
/// executable, or `Err(message)` with a concrete remediation hint.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cli::connect::validate;
///
/// // A healthy ELF binary
/// assert!(validate("PATH=/usr/local/bin/codetether\n\
///     /usr/local/bin/codetether: ELF 64-bit LSB executable\ncodetether 0.5.0")
///     .is_ok());
///
/// // A text file (broken install)
/// assert!(validate("PATH=/usr/local/bin/codetether\n\
///     /usr/local/bin/codetether: ASCII text").is_err());
///
/// // A file with no magic bytes
/// assert!(validate("PATH=/usr/local/bin/codetether\n\
///     /usr/local/bin/codetether: very short file (no magic)").is_err());
/// ```
pub fn validate(report: &str) -> Result<(), String> {
    if report.contains("MISSING") {
        return Err("not found on the remote host — install it on the VM first".into());
    }
    if !report.contains("ELF") {
        return Err(
            "is not a valid ELF executable. The remote file appears to be a broken \
             or partial install (text file, empty placeholder, or truncated download). \
             Reinstall the correct linux binary and retry."
                .into(),
        );
    }
    if report.contains("VERSION_FAILED") {
        return Err(
            "exists and is ELF but `--version` failed — it is not runnable \
             (wrong architecture or missing dynamic linker). \
             Reinstall the correct linux binary and retry."
                .into(),
        );
    }
    Ok(())
}
