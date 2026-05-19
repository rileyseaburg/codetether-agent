//! Session command dispatcher.
//!
//! The runtime keeps feature-gated backend selection out of the public
//! [`BrowserSession`](crate::browser::BrowserSession) type.

/// Execute a browser command against the configured backend.
///
/// # Errors
///
/// Returns [`crate::browser::BrowserError`] when the backend is unavailable or
/// when the requested operation fails.
pub async fn execute(
    session: &crate::browser::BrowserSession,
    command: crate::browser::BrowserCommand,
) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
    #[cfg(feature = "tetherscript")]
    {
        return super::native::execute(session, command).await;
    }

    #[cfg(not(feature = "tetherscript"))]
    {
        let _ = (session, command);
        Err(crate::browser::BrowserError::NotImplemented(
            "browserctl requires the tetherscript feature".into(),
        ))
    }
}
