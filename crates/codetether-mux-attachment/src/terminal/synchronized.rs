//! Atomic terminal presentation while replaying attach-time output.

use std::io::Write;

use anyhow::Result;

const BEGIN: &[u8] = b"\x1b[?2026h";
const END: &[u8] = b"\x1b[?2026l";

pub(super) struct SynchronizedOutput {
    active: bool,
}

impl SynchronizedOutput {
    pub(super) fn begin(active: bool) -> Result<Self> {
        if active {
            let mut stdout = std::io::stdout();
            stdout.write_all(BEGIN)?;
            stdout.flush()?;
        }
        Ok(Self { active })
    }

    pub(super) fn finish(&mut self) -> Result<()> {
        if self.active {
            let mut stdout = std::io::stdout();
            stdout.write_all(END)?;
            stdout.flush()?;
            self.active = false;
        }
        Ok(())
    }
}

impl Drop for SynchronizedOutput {
    fn drop(&mut self) {
        let _ = self.finish();
    }
}
