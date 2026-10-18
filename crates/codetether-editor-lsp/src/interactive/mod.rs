//! Editor click, shortcut, hover and definition interaction policy.
//!
//! [Controls] borrows frontend fields while retaining the application's
//! [ManagedClient] identity. Call [editor_click], [handle_editor_lsp_key],
//! [goto_definition] or [hover] for incoming frontend actions.
use crate::EditorLspState;
use codetether_editor::FileBuffer;
use codetether_lsp_core::manager::{LspManager, ManagedClient};
mod click;
mod definition;
mod hover;
mod keys;
mod navigate;
mod resolve;
pub use click::editor_click;
pub use definition::goto_definition;
pub use hover::hover;
pub use keys::handle_editor_lsp_key;

/// Borrowed editor fields and frontend mode guards used for one interaction.
///
/// # Examples
/// Construct from disjoint frontend fields. The client type preserves manager identity.
/// ```rust,no_run
/// # fn example<C: codetether_lsp_core::manager::ManagedClient>(
/// # editor: &mut Option<codetether_editor::FileBuffer>,
/// # lsp: &mut codetether_editor_lsp::EditorLspState<codetether_lsp_core::manager::LspManager<C>>) {
/// let (mut scroll, mut hscroll, mut status, mut redraw) = (0, 0, String::new(), false);
/// let state = codetether_editor_lsp::interactive::Controls {
///     editor, editor_lsp: lsp, editor_scroll: &mut scroll,
///     editor_hscroll: &mut hscroll, status: &mut status, needs_redraw: &mut redraw,
///     in_editor: true, approval_editing: false, goal_editing: false,
/// };
/// assert!(state.in_editor);
/// # }
/// ```
pub struct Controls<'a, C: ManagedClient> {
    /// Active editor buffer.
    pub editor: &'a mut Option<FileBuffer>,
    /// Vertical viewport offset.
    pub editor_scroll: &'a mut usize,
    /// Horizontal viewport offset.
    pub editor_hscroll: &'a mut usize,
    /// Manager, hover popup and last rendered editor bounds.
    pub editor_lsp: &'a mut EditorLspState<LspManager<C>>,
    /// Frontend status message.
    pub status: &'a mut String,
    /// Frontend redraw flag.
    pub needs_redraw: &'a mut bool,
    /// Whether the current frontend view is the editor.
    pub in_editor: bool,
    /// Whether an approval proposal is being revised.
    pub approval_editing: bool,
    /// Whether the editor holds a goal draft.
    pub goal_editing: bool,
}
