use super::Runner;
use super::upgrade;
use std::path::PathBuf;

fn sandbox_exec() -> PathBuf {
    PathBuf::from("/usr/bin/sandbox-exec")
}

#[test]
fn direct_selection_upgrades_to_seatbelt_when_available() {
    let upgraded = upgrade(
        Runner::Direct("sandbox_exec_not_found"),
        Some(sandbox_exec()),
    );
    assert_eq!(upgraded, Runner::Seatbelt(sandbox_exec()));
}

#[test]
fn direct_selection_stays_direct_without_sandbox_exec() {
    assert_eq!(
        upgrade(Runner::Direct("no_sandbox_backend_on_windows"), None),
        Runner::Direct("no_sandbox_backend_on_windows")
    );
}

#[test]
fn bwrap_selection_is_never_downgraded_to_seatbelt() {
    let bwrap = Runner::Bubblewrap(PathBuf::from("/usr/bin/bwrap"));
    assert_eq!(upgrade(bwrap.clone(), Some(sandbox_exec())), bwrap);
}
