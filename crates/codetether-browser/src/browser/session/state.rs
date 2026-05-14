use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Clone, Default)]
pub struct BrowserSession {
    pub(super) inner: Arc<SessionInner>,
}

#[derive(Default)]
pub(super) struct SessionInner {
    #[cfg(feature = "tetherscript")]
    pub native: Mutex<Option<super::native::NativeRuntime>>,
}

impl BrowserSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub async fn execute(
        &self,
        command: crate::browser::BrowserCommand,
    ) -> Result<crate::browser::BrowserOutput, crate::browser::BrowserError> {
        super::runtime::execute(self, command).await
    }
}
