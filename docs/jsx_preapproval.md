# JSX-aware LSP preapproval

CodeTether chooses a language-server family separately from the LSP document
language ID. TSX and JSX use the same server implementation as TypeScript and
JavaScript, but must not be opened in the plain parser modes.

| File | Server/configuration key | `textDocument/didOpen.languageId` |
| --- | --- | --- |
| `.ts` | `typescript` | `typescript` |
| `.tsx` | `typescript` | `typescriptreact` |
| `.js` | `javascript` | `javascript` |
| `.jsx` | `javascript` | `javascriptreact` |

`src/lsp/document_language.rs` supplies the document ID. Server detection and
workspace overrides still use `detect_language_from_path`, so existing
`[lsp.servers.typescript]` configuration continues to apply to TSX.

The change applies to both on-disk inspection and proposed-content diagnostics.
The preapproval guard remains enabled: real syntax/type errors still produce
`LSP_PREAPPROVAL_FAILED`. No diagnostic codes are filtered out by this change.

Regression tests cover React document IDs, server-family compatibility, actual
TypeScript-server diagnostics, valid TSX/JSX write and patch proposals, malformed
JSX, and a genuine TSX type error. Proposed-content tests leave files unchanged.
Real-server tests require `typescript-language-server` on PATH.

Running agents need the updated binary and fresh LSP connections. This source
fix does not apply an application's inspector patch or restart existing agents.