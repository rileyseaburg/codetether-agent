//! Unit tests for the pure preflight report validator.

use super::preflight_report::validate;

#[test]
fn valid_elf_binary_passes() {
    let report = "PATH=/usr/local/bin/codetether\n\
        /usr/local/bin/codetether: ELF 64-bit LSB pie executable\n\
        codetether 0.5.0";
    assert!(validate(report).is_ok());
}

#[test]
fn missing_binary_is_rejected() {
    let report = "MISSING";
    let err = validate(report).unwrap_err();
    assert!(err.contains("not found"));
}

#[test]
fn text_file_is_rejected() {
    let report = "PATH=/usr/local/bin/codetether\n\
        /usr/local/bin/codetether: ASCII text";
    let err = validate(report).unwrap_err();
    assert!(err.contains("not a valid ELF"));
}

#[test]
fn no_magic_file_is_rejected() {
    let report = "PATH=/usr/local/bin/codetether\n\
        /usr/local/bin/codetether: very short file (no magic)";
    let err = validate(report).unwrap_err();
    assert!(err.contains("not a valid ELF"));
}

#[test]
fn elf_with_failed_version_is_rejected() {
    let report = "PATH=/usr/local/bin/codetether\n\
        /usr/local/bin/codetether: ELF 64-bit LSB executable\n\
        VERSION_FAILED";
    let err = validate(report).unwrap_err();
    assert!(err.contains("--version"));
}
