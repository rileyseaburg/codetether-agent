//! Invalid requests must fail before queue execution.
use super::super::validate;
use super::*;

#[test]
fn invalid_target_action_and_modifiers() {
    for value in [
        json!({"action":"click"}),
        json!({"action":"status","hwnd":0}),
        json!({"action":"status","hwnd":-1}),
        json!({"action":"bring_to_front","hwnd":1}),
        json!({"action":"click","hwnd":1,"modifiers":["shift"]}),
    ] {
        assert!(validate::request(&input(value)).is_err());
    }
}
#[test]
fn coordinate_and_drag_bounds() {
    for extra in [
        json!({"x":1}),
        json!({"x":1.1,"y":2}),
        json!({"steps":0}),
        json!({"steps":241}),
        json!({"duration_ms":30001}),
    ] {
        let mut value = json!({"action":"drag","hwnd":1,"x2":5,"y2":6});
        value
            .as_object_mut()
            .unwrap()
            .extend(extra.as_object().unwrap().clone());
        assert!(validate::request(&input(value)).is_err());
    }
    let mut value = input(json!({"action":"click","hwnd":1,"x":0,"y":0}));
    value.x = Some(f64::NAN);
    assert!(validate::request(&value).is_err());
}
#[test]
fn invalid_scroll_and_text() {
    for value in [
        json!({"action":"scroll","hwnd":1,"scroll_amount":32768}),
        json!({"action":"type_text","hwnd":1}),
        json!({"action":"type_text","hwnd":1,"text":"\0"}),
        json!({"action":"type_text","hwnd":1,"text":"a".repeat(8193)}),
    ] {
        assert!(validate::request(&input(value)).is_err());
    }
}
