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
