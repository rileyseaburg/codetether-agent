//! Context-sensitive title for the chat input box.

use crate::ComposerView;

const COMPACT_WIDTH: u16 = 80;

/// Build a width- and environment-sensitive input title.
///
/// # Arguments
/// * `view` - Processing and command-mode flags.
/// * `suffix` - Text appended to the title.
/// * `width` - Available input-panel width.
///
/// # Returns
/// An owned title with relevant input shortcuts.
///
/// # Examples
/// ```
/// let view = codetether_composer_view::ComposerView { text: "hello", cursor: 5, processing: false, command_mode: false };
/// let title = codetether_composer_view::build_title(&view, "", 44);
/// assert!(title.contains("Enter send"));
/// ```
pub fn build(view: &ComposerView<'_>, suffix: &str, width: u16) -> String {
    let compact = width < COMPACT_WIDTH;
    if view.processing {
        if compact {
            return format!(" Enter steers · Esc cancels{suffix}");
        }
        return format!(" Message (Processing — Enter steers turn · Esc cancels){suffix}");
    }
    if view.command_mode {
        return format!(" Command (/ for commands, Tab to autocomplete){suffix}");
    }
    if compact {
        return format!(" Message (Enter send · / cmds · ? help){suffix}");
    }
    let paste = if codetether_terminal_clipboard::is_ssh_or_headless() {
        "Ctrl+Shift+V"
    } else {
        "Ctrl+V"
    };
    format!(
        " Message (Enter=send · {paste}=paste · Ctrl+O=copy reply · \
         Ctrl+⇧Y=copy all · Ctrl+R=voice){suffix}"
    )
}
