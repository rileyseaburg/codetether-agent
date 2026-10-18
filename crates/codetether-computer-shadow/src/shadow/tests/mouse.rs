//! Click sequencing, bounded drag, and staged logical pointer tests.
use super::*;

#[test]
fn click_and_double_message_sequences() {
    for (action, ids) in [
        ("click", vec![0x200, 0x201, 0x202]),
        ("right_click", vec![0x200, 0x204, 0x205]),
        ("double_click", vec![0x200, 0x201, 0x202, 0x203, 0x202]),
    ] {
        let plan = planned(json!({"action":action,"hwnd":1,"x":4,"y":5}));
        assert_eq!(
            plan.events.iter().map(|e| e.message).collect::<Vec<_>>(),
            ids
        );
        assert_eq!(plan.state.buttons, 0);
    }
}
#[test]
fn drag_is_bounded_and_reaches_endpoint() {
    let plan = planned(
        json!({"action":"drag","hwnd":1,"x":-32768,"y":0,"x2":32767,"y2":6,"steps":240,"duration_ms":30000}),
    );
    assert_eq!(plan.events.len(), 243);
    assert_eq!(plan.events.iter().map(|e| e.delay_ms).sum::<u64>(), 30000);
    assert_eq!(plan.state.pointer_client.unwrap().x, 32767);
    assert_eq!(plan.state.buttons, 0);
}
#[test]
fn staged_pointer_reuse_and_unheld_release_rejection() {
    let down = planned(json!({"action":"mouse_down","hwnd":1,"x":4,"y":5}));
    let up = input(json!({"action":"mouse_up","hwnd":1}));
    let plan = plan::build(&up, Geometry::default(), down.state).unwrap();
    assert_eq!(plan.state.buttons, 0);
    assert!(plan::build(&up, Geometry::default(), State::default()).is_err());
    let stop = input(json!({"action":"stop","hwnd":1}));
    let releases = plan::build(&stop, Geometry::default(), down.state).unwrap();
    assert_eq!(releases.events.len(), 1);
    assert_eq!(releases.events[0].message, 0x202);
    assert!(
        plan::build(&stop, Geometry::default(), State::default())
            .unwrap()
            .events
            .is_empty()
    );
}
