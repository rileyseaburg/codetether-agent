use super::*;
use std::path::PathBuf;

#[test]
fn selection_prefers_bwrap_on_linux_when_present() {
    let bwrap = PathBuf::from("/usr/bin/bwrap");
    assert_eq!(
        select_for(true, Some(bwrap.clone())),
        Runner::Bubblewrap(bwrap)
    );
    assert_eq!(select_for(true, None), Runner::Direct("bwrap_not_found"));
}

#[test]
fn non_linux_selection_names_the_missing_backend() {
    assert_eq!(
        select_for(false, None),
        Runner::Direct(platform::unsupported_reason())
    );
}
