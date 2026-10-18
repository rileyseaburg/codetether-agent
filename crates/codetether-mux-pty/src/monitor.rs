//! Registration of PTY children with the process-wide child reaper.

use std::process::Child;
use std::sync::{OnceLock, mpsc};

mod reaper;
#[cfg(all(test, target_os = "linux"))]
mod tests;

static REAPER: OnceLock<mpsc::Sender<Child>> = OnceLock::new();

pub(super) fn start(child: Child) {
    if let Err(error) = sender().send(child) {
        std::thread::spawn(move || {
            let mut child = error.0;
            let _ = child.wait();
        });
    }
}

fn sender() -> &'static mpsc::Sender<Child> {
    REAPER.get_or_init(|| {
        let (sender, receiver) = mpsc::channel();
        std::thread::Builder::new()
            .name("codetether-child-reaper".into())
            .spawn(move || reaper::run(receiver))
            .expect("spawn PTY child reaper");
        sender
    })
}
