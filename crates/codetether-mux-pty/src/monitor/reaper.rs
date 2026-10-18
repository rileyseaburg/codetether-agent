//! Blocking child collection shared by every mux PTY program.

use std::process::Child;
use std::sync::mpsc::{Receiver, RecvTimeoutError};
use std::time::Duration;

const POLL_INTERVAL: Duration = Duration::from_millis(100);

pub(super) fn run(receiver: Receiver<Child>) {
    let mut children = Vec::new();
    while let Ok(child) = receiver.recv() {
        children.push(child);
        reap_until_empty(&receiver, &mut children);
    }
}

fn reap_until_empty(receiver: &Receiver<Child>, children: &mut Vec<Child>) {
    while !children.is_empty() {
        match receiver.recv_timeout(POLL_INTERVAL) {
            Ok(child) => children.push(child),
            Err(RecvTimeoutError::Timeout) => {}
            Err(RecvTimeoutError::Disconnected) => return,
        }
        while let Ok(child) = receiver.try_recv() {
            children.push(child);
        }
        children.retain_mut(still_running);
    }
}

fn still_running(child: &mut Child) -> bool {
    matches!(child.try_wait(), Ok(None))
}
