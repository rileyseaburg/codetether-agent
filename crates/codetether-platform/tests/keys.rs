//! Pure Windows key parsing can be verified without sending input.
#[path = "../src/windows/computer_use/input/vk_table.rs"]
mod vk_table;
#[path = "../src/windows/computer_use/input/parse.rs"]
mod parse;

#[test]
fn modifiers_named_keys_and_fallbacks_match_the_input_contract() {
    assert_eq!(parse::parse_send_keys("^c"), [0x11, 0x43]);
    assert_eq!(parse::parse_send_keys("+%{TAB}"), [0x10, 0x12, 0x09]);
    assert_eq!(parse::parse_send_keys("CTRL SHIFT Z"), [0x11, 0x10, 0x5a]);
    assert_eq!(parse::parse_send_keys(""), [0x0d]);
    assert_eq!(parse::parse_send_keys("unknown"), [0x0d]);
    assert_eq!(vk_table::resolve_vk("NUMPAD_DOT"), 0x6e);
}
