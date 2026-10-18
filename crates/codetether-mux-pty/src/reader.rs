//! Background PTY output capture.

use std::fs::File;
use std::io::Read;
use std::sync::{
    Arc, Mutex,
    atomic::{AtomicBool, Ordering},
};

use super::buffer::OutputBuffer;

pub(super) fn start(
    mut reader: File,
    output: Arc<Mutex<OutputBuffer>>,
    running: Arc<AtomicBool>,
    changed: tokio::sync::watch::Sender<u64>,
) {
    std::thread::spawn(move || {
        let mut bytes = [0_u8; 8192];
        loop {
            match reader.read(&mut bytes) {
                Ok(0) | Err(_) => break,
                Ok(count) => {
                    output.lock().unwrap().append(&bytes[..count]);
                    changed.send_modify(|version| *version += 1);
                }
            }
        }
        running.store(false, Ordering::Release);
        changed.send_modify(|version| *version += 1);
    });
}
