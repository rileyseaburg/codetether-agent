//! Child environment tests for mux identity inheritance.

use std::ffi::OsStr;

#[test]
fn configured_child_inherits_mux_session_identity() {
    let slave = std::fs::File::open("/dev/null").unwrap();
    let command = super::configured("true", std::path::Path::new("/tmp"), slave, "team").unwrap();
    let inherited = command
        .get_envs()
        .find(|(name, _)| *name == OsStr::new(crate::mux::coordination::SESSION_ENV))
        .and_then(|(_, value)| value);
    assert_eq!(inherited, Some(OsStr::new("team")));
}
