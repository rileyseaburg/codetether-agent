//! Bounded pairing result ownership; no heap pointers in window messages.
use crate::relay::Device;
use std::thread::JoinHandle;

pub(super) struct PairTask {
    pub(super) valid: bool,
    pub(super) thread: JoinHandle<Result<Device, String>>,
}
