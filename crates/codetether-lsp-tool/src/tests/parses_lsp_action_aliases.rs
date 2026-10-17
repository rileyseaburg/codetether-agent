//! Preserved LSP argument regression.
use crate::LspOperation;

#[test]
fn parses_lsp_action_aliases() {
    assert_eq!(
        LspOperation::parse("document-symbol"),
        Some(LspOperation::DocumentSymbol)
    );
    assert_eq!(
        LspOperation::parse("workspace_symbol"),
        Some(LspOperation::WorkspaceSymbol)
    );
    assert_eq!(
        LspOperation::parse("goToImplementation"),
        Some(LspOperation::GoToImplementation)
    );
    assert_eq!(
        LspOperation::parse("diagnostics"),
        Some(LspOperation::Diagnostics)
    );
    assert_eq!(LspOperation::parse("unknown"), None);
}
