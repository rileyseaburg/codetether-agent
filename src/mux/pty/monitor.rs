//! PTY child lifetime monitoring.

use std::process::Child;

pub(super) fn start(mut child: Child) {
    std::thread::spawn(move || {
        let _ = child.wait();
    });
}
