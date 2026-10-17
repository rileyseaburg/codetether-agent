//! Supply application construction and initialization to the shared manager.
use super::*;
impl codetether_lsp_core::manager::ManagedClient for LspClient {
    fn new(config: LspConfig) -> impl std::future::Future<Output = Result<Self>> + Send {
        LspClient::new(config)
    }
    fn for_language(
        language: &str,
        root_uri: Option<String>,
    ) -> impl std::future::Future<Output = Result<Self>> + Send {
        LspClient::for_language(language, root_uri)
    }
    fn initialize(&self) -> impl std::future::Future<Output = Result<()>> + Send {
        LspClient::initialize(self)
    }
}
