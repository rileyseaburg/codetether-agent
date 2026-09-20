//! Resolve clipboard shortcuts in the mux client, where the clipboard lives.
//!
//! The detached server-side TUI may not share the client's X11/Wayland
//! environment. Converting local Ctrl+V into a bracketed-paste payload keeps
//! text sidecars and image attachments identical to a non-mux TUI.

#[path = "clipboard/stream.rs"]
mod stream;
pub(in crate::mux::client::proxy) use stream::Resolver;

const CTRL_V: u8 = 0x16;
pub(super) const START: &[u8] = b"\x1b[200~";
pub(super) const END: &[u8] = b"\x1b[201~";

fn resolve_content(data: Vec<u8>, content: Option<String>) -> Vec<u8> {
    let Some(content) = content else {
        return frame_unmarked_paste(data);
    };
    let mut resolved = Vec::with_capacity(data.len() + content.len());
    for value in data {
        if value == CTRL_V {
            resolved.extend(frame(content.as_bytes()));
        } else {
            resolved.push(value);
        }
    }
    resolved
}

fn frame_unmarked_paste(data: Vec<u8>) -> Vec<u8> {
    let split = data.iter().position(|value| matches!(value, b'\r' | b'\n'));
    let multiline = split.is_some_and(|index| {
        data[..index].iter().any(u8::is_ascii_graphic)
            && data[index + 1..].iter().any(u8::is_ascii_graphic)
    });
    if multiline { frame(&data) } else { data }
}

fn local_content() -> Option<String> {
    crate::tui::clipboard::get_clipboard_text()
        .map(|text| text.replace("\r\n", "\n").replace('\r', "\n"))
        .or_else(|| crate::tui::clipboard::get_clipboard_image().map(|image| image.data_url))
}

fn frame(content: &[u8]) -> Vec<u8> {
    let mut framed = Vec::with_capacity(START.len() + content.len() + END.len());
    framed.extend_from_slice(START);
    framed.extend_from_slice(content);
    framed.extend_from_slice(END);
    framed
}

#[cfg(test)]
#[path = "clipboard_tests.rs"]
mod tests;
