//! LSP tool specialized for the application's initialization identity.
/// LSP tool using the existing application client wrapper.
///
/// # Examples
/// ```rust
/// let _tool = codetether_agent::tool::lsp::LspTool::new();
/// ```
pub type LspTool = codetether_lsp_tool::LspTool<ApplicationHost>;
/// Supplies the existing client identity and one process-wide manager cache.
///
/// # Examples
/// ```rust
/// let _host = codetether_agent::tool::lsp::ApplicationHost;
/// ```
pub struct ApplicationHost;
impl codetether_lsp_tool::Host for ApplicationHost {
    type Client = crate::lsp::client::LspClient;
    fn state() -> &'static codetether_lsp_tool::State<Self::Client> {
        static STATE: codetether_lsp_tool::State<crate::lsp::client::LspClient> =
            codetether_lsp_tool::State::new();
        &STATE
    }
}
