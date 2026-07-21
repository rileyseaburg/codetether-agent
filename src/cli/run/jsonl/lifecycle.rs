use super::event::RunEvent;
use crate::provider::Usage;
use anyhow::Result;

pub(in crate::cli::run) fn write_started(session_id: &str) -> Result<()> {
    super::writer::write_stdout(&RunEvent::Started { session_id })
}

pub(in crate::cli::run) fn write_completed(
    session_id: Option<&str>,
    response: &str,
    usage: Option<&Usage>,
) -> Result<()> {
    super::writer::write_stdout(&RunEvent::Completed {
        session_id,
        response,
        usage,
    })
}

pub(in crate::cli::run) fn write_failed(error: impl AsRef<str>) -> Result<()> {
    write_failed_response(error.as_ref(), None)
}

pub(in crate::cli::run) fn write_failed_response(
    error: &str,
    response: Option<&str>,
) -> Result<()> {
    super::writer::write_stdout(&RunEvent::Failed { error, response })
}
