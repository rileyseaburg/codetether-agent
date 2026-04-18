mod runtime;

#[derive(Clone, Default)]
pub struct BrowserSession;

impl BrowserSession {
    pub fn new() -> Self {
        Self
    }

    pub async fn execute(
        &self,
        command: crate::browser::BrowserCommand,
    ) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
        runtime::execute(command).await
    }
}
