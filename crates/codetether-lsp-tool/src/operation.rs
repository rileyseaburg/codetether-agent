//! The `lsp` tool's action vocabulary: parsing aliases and per-action rules.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LspOperation {
    GoToDefinition,
    FindReferences,
    Hover,
    DocumentSymbol,
    WorkspaceSymbol,
    GoToImplementation,
    Completion,
    Diagnostics,
}

const ALL: [LspOperation; 8] = [
    LspOperation::GoToDefinition,
    LspOperation::FindReferences,
    LspOperation::Hover,
    LspOperation::DocumentSymbol,
    LspOperation::WorkspaceSymbol,
    LspOperation::GoToImplementation,
    LspOperation::Completion,
    LspOperation::Diagnostics,
];

impl LspOperation {
    /// Accepts the canonical camelCase name plus its kebab-case and
    /// snake_case forms (`goToDefinition`, `go-to-definition`, `go_to_definition`).
    pub(super) fn parse(action: &str) -> Option<Self> {
        let wanted = action.replace(['-', '_'], "").to_ascii_lowercase();
        ALL.into_iter()
            .find(|op| op.canonical_name().to_ascii_lowercase() == wanted)
    }

    pub(super) fn requires_position(self) -> bool {
        match self {
            Self::GoToDefinition
            | Self::FindReferences
            | Self::Hover
            | Self::GoToImplementation
            | Self::Completion => true,
            Self::DocumentSymbol | Self::WorkspaceSymbol | Self::Diagnostics => false,
        }
    }

    pub(super) fn canonical_name(self) -> &'static str {
        match self {
            Self::GoToDefinition => "goToDefinition",
            Self::FindReferences => "findReferences",
            Self::Hover => "hover",
            Self::DocumentSymbol => "documentSymbol",
            Self::WorkspaceSymbol => "workspaceSymbol",
            Self::GoToImplementation => "goToImplementation",
            Self::Completion => "completion",
            Self::Diagnostics => "diagnostics",
        }
    }
}
