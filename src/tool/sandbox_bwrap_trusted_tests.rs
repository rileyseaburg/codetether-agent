//! Trusted Bubblewrap resolution tests.

use super::{CANDIDATES, find};

#[test]
fn selected_binary_is_from_a_pinned_system_location() {
    let Some(selected) = find() else { return };
    assert!(CANDIDATES.iter().any(|candidate| {
        std::path::Path::new(candidate)
            .canonicalize()
            .is_ok_and(|path| path == selected)
    }));
}

#[test]
fn inherited_path_cannot_substitute_bubblewrap() {
    let _lock = crate::approval::test_env::lock_env();
    let temp = tempfile::tempdir().unwrap();
    let fake = temp.path().join("bwrap");
    std::fs::write(&fake, "not bubblewrap").unwrap();
    let old = std::env::var_os("PATH");
    unsafe { std::env::set_var("PATH", temp.path()) };

    let selected = find();

    match old {
        Some(value) => unsafe { std::env::set_var("PATH", value) },
        None => unsafe { std::env::remove_var("PATH") },
    }
    assert_ne!(
        selected,
        fake.canonicalize().ok(),
        "inherited PATH selected an untrusted sandbox binary"
    );
}