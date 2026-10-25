//! First terminal capture error shared by CPAL callbacks and the recorder.
use anyhow::{Result, anyhow};
use cpal::{Error, ErrorKind};
use std::sync::{Arc, Mutex};

#[derive(Clone, Default)]
pub(crate) struct CaptureErrors(Arc<Mutex<Option<String>>>);

impl CaptureErrors {
    pub(crate) fn report(&self, error: Error) {
        let reason = error.to_string();
        if matches!(
            error.kind(),
            ErrorKind::DeviceChanged | ErrorKind::RealtimeDenied | ErrorKind::Xrun
        ) {
            tracing::warn!(error = %reason, "Audio capture stream notification");
            return;
        }
        tracing::error!(error = %reason, "Audio capture stream failed");
        if let Ok(mut first) = self.0.lock()
            && first.is_none()
        {
            *first = Some(reason);
        }
    }

    pub(crate) fn check(&self) -> Result<()> {
        let first = self
            .0
            .lock()
            .map_err(|_| anyhow!("Audio capture error lock poisoned"))?;
        match first.as_deref() {
            Some(reason) => Err(anyhow!("Audio capture failed: {reason}")),
            None => Ok(()),
        }
    }
}
