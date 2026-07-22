use std::ffi::OsString;
use std::path::Path;

use super::select;

#[test]
fn replaced_executable_uses_configured_binary() {
    let selected = select(
        Path::new("/missing/codetether (deleted)"),
        Some(OsString::from("/installed/codetether")),
    );

    assert_eq!(selected, OsString::from("/installed/codetether"));
}

#[test]
fn replaced_executable_uses_path_lookup_by_default() {
    let selected = select(Path::new("/missing/codetether (deleted)"), None);

    assert_eq!(selected, OsString::from("codetether"));
}
