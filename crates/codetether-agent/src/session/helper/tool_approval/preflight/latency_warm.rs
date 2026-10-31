//! Load Node and the fixture script outside the measured diagnostic window.
use std::{path::Path, process::Command};

pub(super) fn script(node: &Path, script: &Path) {
    let status = Command::new(node)
        .arg("--check")
        .arg(script)
        .status()
        .expect("fake LSP script must load before the latency test");
    assert!(status.success(), "fake LSP syntax check failed");
}
