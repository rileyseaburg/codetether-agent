//! Data types for bounded bash output capture.

use std::process::ExitStatus;

pub(crate) struct CapturedStream {
    pub(crate) text: String,
    pub(crate) bytes: usize,
    pub(crate) truncated: bool,
}

impl CapturedStream {
    pub(crate) fn empty() -> Self {
        Self {
            text: String::new(),
            bytes: 0,
            truncated: false,
        }
    }
}

pub(crate) struct CapturedOutput {
    pub(crate) status: ExitStatus,
    pub(crate) stdout: CapturedStream,
    pub(crate) stderr: CapturedStream,
}

impl CapturedOutput {
    pub(crate) fn total_bytes(&self) -> usize {
        self.stdout.bytes + self.stderr.bytes
    }

    pub(crate) fn truncated(&self) -> bool {
        self.stdout.truncated || self.stderr.truncated
    }
}

pub(crate) enum CaptureOutcome {
    Finished(CapturedOutput),
    TimedOut,
}
