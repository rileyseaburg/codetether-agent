//! Unicode surrogate and safe key-message tests.
use super::*;

#[test]
fn unicode_uses_utf16_char_messages() {
    let plan = planned(json!({"action":"type_text","hwnd":1,"text":"A😀"}));
    assert_eq!(
        plan.events.iter().map(|e| e.wparam).collect::<Vec<_>>(),
        [65, 0xd83d, 0xde00]
    );
    assert!(
        plan.events
            .iter()
            .all(|e| e.message == 0x0102 && e.lparam == 1)
    );
}
#[test]
fn keys_have_transition_scan_and_extended_flags() {
    let plan = planned(json!({"action":"press_key","hwnd":1,"key":"Left"}));
    assert_eq!(plan.events[0].message, 0x0100);
    assert_eq!(plan.events[0].lparam as u32, 0x014b0001);
    assert_eq!(plan.events[1].message, 0x0101);
    assert_eq!(plan.events[1].lparam as u32, 0xc14b0001);
    assert!(plan.events[0].after.pending_key.is_some());
    assert!(plan.events[1].after.pending_key.is_none());
}
#[test]
fn rejects_chords_and_sendkeys() {
    for key in ["^c", "Ctrl+C", "shift", "%{TAB}", "a", "{ENTER}"] {
        let request = input(json!({"action":"press_key","hwnd":1,"key":key}));
        assert!(plan::build(&request, Geometry::default(), State::default()).is_err());
    }
}
