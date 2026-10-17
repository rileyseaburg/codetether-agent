//! Capability gate: diagnostics-only servers reject non-diagnostic actions.

use lsp_types::{HoverProviderCapability, OneOf, ServerCapabilities, TextDocumentSyncKind};

use super::LspOperation;
use super::capability_gate::check;

/// What `tetherscript lsp` (0.1.0-alpha.31) actually advertises.
fn diagnostics_only() -> ServerCapabilities {
    ServerCapabilities {
        text_document_sync: Some(TextDocumentSyncKind::FULL.into()),
        ..Default::default()
    }
}

#[test]
fn diagnostics_only_server_allows_diagnostics() {
    assert!(check(LspOperation::Diagnostics, Some(&diagnostics_only())).is_ok());
}

#[test]
fn diagnostics_only_server_refuses_hover_and_definition_with_a_hint() {
    for action in [
        LspOperation::Hover,
        LspOperation::GoToDefinition,
        LspOperation::DocumentSymbol,
    ] {
        let reason = check(action, Some(&diagnostics_only())).unwrap_err();
        assert!(reason.contains(action.canonical_name()), "{reason}");
        assert!(reason.contains("`diagnostics`"), "{reason}");
    }
}

#[test]
fn full_server_passes_and_unknown_capabilities_are_not_gated() {
    let full = ServerCapabilities {
        hover_provider: Some(HoverProviderCapability::Simple(true)),
        definition_provider: Some(OneOf::Left(true)),
        ..Default::default()
    };
    assert!(check(LspOperation::Hover, Some(&full)).is_ok());
    assert!(check(LspOperation::GoToDefinition, Some(&full)).is_ok());
    assert!(check(LspOperation::Hover, None).is_ok());
}
