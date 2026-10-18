//! Timing helpers for Blender UI automation.

use std::{thread::sleep, time::Duration};

const DIALOG_MS: u64 = 500;
const SETTLE_MS: u64 = 120;
const PROBE_MS: u64 = 100;

pub fn dialog() {
    sleep(Duration::from_millis(DIALOG_MS));
}

pub fn settle() {
    sleep(Duration::from_millis(SETTLE_MS));
}

pub fn probe() {
    sleep(Duration::from_millis(PROBE_MS));
}
