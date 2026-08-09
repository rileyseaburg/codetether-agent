//! Resolve clipboard shortcuts in the mux client, where the clipboard lives.
//!
//! The detached server-side TUI may not share the client's X11/Wayland
//! environment. Converting local Ctrl+V into a bracketed-paste payload keeps
//! text sidecars and image attachments identical to a non-mux TUI.

const CTRL_V: u8 = 0x16;
const START: &[u8] = b"\x1b[200~";
const END: &[u8] = b"\x1b[201~";

/// Replace a standalone Ctrl+V with local clipboard content when available.
pub(super) fn resolve(data: Vec<u8>) -> Vec<u8> {
    resolve_content(data, local_content())
}

fn resolve_content(data: Vec<u8>, content: Option<String>) -> Vec<u8> {
    if data.as_slice() != [CTRL_V] {
        return data;
    }
    content.map_or(data, |value| frame(value.as_bytes()))
}

fn local_content() -> Option<String> {
    crate::tui::clipboard::get_clipboard_text()
        .map(normalize)
        .or_else(|| crate::tui::clipboard::get_clipboard_image().map(|image| image.data_url))
}

fn normalize(text: String) -> String {
    text.replace("\r\n", "\n").replace('\r', "\n")
}

fn frame(content: &[u8]) -> Vec<u8> {
    let mut framed = Vec::with_capacity(START.len() + content.len() + END.len());
    framed.extend_from_slice(START);
    framed.extend_from_slice(content);
    framed.extend_from_slice(END);
    framed
}

#[cfg(test)]
mod tests {
    use super::{CTRL_V, frame, resolve, resolve_content};

    #[test]
    fn frames_content_as_one_terminal_paste() {
        assert_eq!(frame(b"one\ntwo"), b"\x1b[200~one\ntwo\x1b[201~");
    }

    #[test]
    fn leaves_non_shortcut_input_unchanged() {
        assert_eq!(resolve(vec![CTRL_V, b'x']), vec![CTRL_V, b'x']);
    }

    #[test]
    fn standalone_ctrl_v_uses_local_clipboard_content() {
        assert_eq!(
            resolve_content(vec![CTRL_V], Some("one\ntwo".into())),
            b"\x1b[200~one\ntwo\x1b[201~"
        );
    }
}
