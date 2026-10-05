//! Missing-session classification shared by full and bounded context readers.
use crate::session::Session;
use anyhow::Result;
pub(super) fn classify(result: Result<Session>) -> Result<Option<Session>> {
    match result {
        Ok(session) => Ok(Some(session)),
        Err(error) => {
            let message = error.to_string().to_lowercase();
            if message.contains("no session")
                || message.contains("not found")
                || message.contains("no such file")
            {
                Ok(None)
            } else {
                Err(error)
            }
        }
    }
}
