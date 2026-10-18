//! Optional real-server regressions for the shared LSP client and manager.
//!
//! Tests retain their existing tool-availability guards.
//! The fixture identity keeps these diagnostics tests independent of native UI code.
use codetether_lsp_core::{
    path_to_uri,
    types::{DiagnosticInfo, LspActionResult, ServerInfo},
};
struct Identity;
impl crate::Host for Identity {
    fn version() -> &'static str {
        "test"
    }
    fn initialized(_server_info: &Option<ServerInfo>) {}
}
type LspClient = crate::LspClient<Identity>;
type LspManager = crate::LspManager<LspClient>;
mod jsx_diagnostics_tests;
mod proposed_diagnostics_tests;
mod tetherscript_smoke_tests;
mod typescript_repeated_tests;
