//! Owns one worker thread; the UI retains it until capture cleanup finishes.
use super::{work_loop, work_stop::Stop};
use crate::relay::Device;
use codetether_companion_desktop::Monitor;
use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, AtomicU8, Ordering},
    },
    thread::JoinHandle,
};

pub(super) enum Exit {
    Cancelled,
    Suspended,
    Revoked,
    Failed,
}
pub(super) struct Worker {
    pub(super) stop: Stop,
    pub(super) join: JoinHandle<Exit>,
    phase: Arc<AtomicU8>,
    pub(super) device: Arc<Device>,
}
impl Worker {
    pub(super) fn spawn(
        device: Arc<Device>,
        monitor: Monitor,
        flag: Arc<AtomicBool>,
    ) -> std::io::Result<Self> {
        let stop = Stop::new(flag);
        let worker_stop = stop.clone();
        let phase = Arc::new(AtomicU8::new(0));
        let worker_phase = phase.clone();
        let work_device = device.clone();
        let join = std::thread::Builder::new()
            .name("companion-requests".into())
            .spawn(move || {
                let Ok(runtime) = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                else {
                    return Exit::Failed;
                };
                let result = runtime.block_on(work_loop::run(
                    &work_device,
                    &monitor,
                    &worker_stop,
                    &worker_phase,
                ));
                worker_stop.cancel();
                runtime.block_on(super::work_finish::notify(&work_device));
                // Runtime drop waits for a cancelled native capture to release resources.
                drop(runtime);
                result
            })?;
        Ok(Self {
            stop,
            join,
            phase,
            device,
        })
    }
    pub(super) fn status(&self) -> &'static str {
        match self.phase.load(Ordering::Acquire) {
            1 => "Sharing ON — capturing the selected monitor for your iPhone.",
            2 => "Sharing ON — uploading the requested screenshot.",
            3 => "Sharing ON — relay unavailable or busy; retrying.",
            4 => "Keyboard delivery — typing directly at your cursor. Escape or Pause stops it.",
            5 => "Keyboard events sent — check the target text. No Enter/submit key was sent.",
            6 => {
                "Typing refused or interrupted — check the target for partial text before resending."
            }
            7 => {
                "Typing refused — use single-line plain text, at most 2000 UTF-16 units; no tabs/newlines."
            }
            _ => "Sharing ON — ready for screenshots and keyboard replies from your iPhone.",
        }
    }
}
