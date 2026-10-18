//! Apply shared editor buffer navigation to application fields.
use super::Controls;
use codetether_lsp_core::LocationInfo;
use codetether_lsp_core::manager::ManagedClient;
pub(super) fn navigate(state: &mut Controls<'_, impl ManagedClient>, loc: &LocationInfo) {
    if let Some(status) =
        crate::navigate(state.editor, state.editor_scroll, state.editor_hscroll, loc)
    {
        *state.status = status;
    }
}
