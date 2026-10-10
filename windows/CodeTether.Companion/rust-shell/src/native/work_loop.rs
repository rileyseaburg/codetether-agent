//! Serialized request polling with bounded retry and shutdown.
use super::{
    work_cycle::{self, Issue},
    work_stop::Stop,
    worker::Exit,
};
use crate::relay::{Device, Error};
use codetether_companion_desktop::Monitor;
use std::{
    sync::{
        Arc,
        atomic::{AtomicU8, Ordering},
    },
    time::Duration,
};

pub(super) async fn run(
    device: &Device,
    monitor: &Monitor,
    stop: &Stop,
    phase: &Arc<AtomicU8>,
) -> Exit {
    tokio::select! { biased;
        _ = stop.watch() => Exit::Cancelled,
        result = requests(device, monitor, stop, phase) => result,
    }
}
async fn requests(device: &Device, monitor: &Monitor, stop: &Stop, phase: &Arc<AtomicU8>) -> Exit {
    let mut last = None;
    loop {
        if phase.load(Ordering::Acquire) < 5 {
            phase.store(0, Ordering::Release);
        }
        let delay = match work_cycle::cycle(device, monitor, stop, phase, last.as_deref()).await {
            Ok(id) => {
                if id.is_some() {
                    last = id;
                }
                2
            }
            Err(Issue::Desktop) => return Exit::Suspended,
            Err(Issue::Relay(Error::Cancelled)) => return Exit::Cancelled,
            Err(Issue::Relay(Error::Revoked)) => return Exit::Revoked,
            Err(Issue::Relay(Error::InvalidInput | Error::InvalidResponse)) => return Exit::Failed,
            Err(Issue::Relay(Error::Conflict | Error::Rejected(_) | Error::Unavailable)) => {
                phase.store(3, Ordering::Release);
                5
            }
        };
        tokio::time::sleep(Duration::from_secs(delay)).await;
    }
}
