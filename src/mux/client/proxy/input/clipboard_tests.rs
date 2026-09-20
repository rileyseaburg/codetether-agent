use super::{CTRL_V, Resolver, frame, resolve_content};

#[test]
fn frames_content_as_one_terminal_paste() {
    assert_eq!(frame(b"one\ntwo"), b"\x1b[200~one\ntwo\x1b[201~");
}

#[test]
fn leaves_existing_bracketed_paste_unchanged() {
    let mut resolver = Resolver::default();
    let paste = b"\x1b[200~one\ntwo\x1b[201~".to_vec();
    assert_eq!(resolver.resolve(paste.clone()), paste);
}

#[test]
fn unmarked_multiline_input_becomes_one_paste() {
    assert_eq!(
        resolve_content(b"one\ntwo".to_vec(), None),
        b"\x1b[200~one\ntwo\x1b[201~"
    );
}

#[test]
fn enter_key_remains_an_enter_key() {
    assert_eq!(resolve_content(vec![b'\r'], None), vec![b'\r']);
}

#[test]
fn typed_command_and_enter_are_not_reclassified() {
    let command = b"status\r".to_vec();
    assert_eq!(resolve_content(command.clone(), None), command);
}

#[test]
fn ctrl_v_inside_input_uses_local_clipboard_content() {
    assert_eq!(
        resolve_content(vec![b'a', CTRL_V, b'z'], Some("one\ntwo".into())),
        b"a\x1b[200~one\ntwo\x1b[201~z"
    );
}

#[test]
fn missing_clipboard_preserves_ctrl_v() {
    assert_eq!(resolve_content(vec![CTRL_V], None), vec![CTRL_V]);
}

#[test]
fn ordinary_input_is_unchanged() {
    assert_eq!(resolve_content(b"status".to_vec(), None), b"status");
}
