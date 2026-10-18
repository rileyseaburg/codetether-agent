//! Screen/client distinction and signed packing regression tests.
use super::super::{coordinates::pack, types::Point};
use super::*;

#[test]
fn signed_coordinate_packing() {
    assert_eq!(pack(Point { x: -1, y: -2 }).unwrap() as u32, 0xfffe_ffff);
    assert!(pack(Point { x: 32768, y: 0 }).is_err());
    assert!(pack(Point { x: 0, y: -32769 }).is_err());
}
#[test]
fn outer_client_and_wheel_screen_coordinates() {
    let geometry = Geometry {
        outer: Point { x: 100, y: 200 },
        client: Point { x: 108, y: 230 },
    };
    let mut request = input(json!({"action":"click","hwnd":1,"x":18,"y":40}));
    let click = plan::build(&request, geometry, State::default()).unwrap();
    assert_eq!(
        click.events[0].lparam,
        pack(Point { x: 10, y: 10 }).unwrap()
    );
    request.action = super::super::super::input::ComputerUseAction::Scroll;
    request.scroll_amount = Some(-120);
    let wheel = plan::build(&request, geometry, State::default()).unwrap();
    assert_eq!(
        wheel.events[0].lparam,
        pack(Point { x: 118, y: 240 }).unwrap()
    );
    assert_eq!((wheel.events[0].wparam >> 16) as u16 as i16, -120);
    request.client_area = true;
    let wheel = plan::build(&request, geometry, State::default()).unwrap();
    assert_eq!(
        wheel.events[0].lparam,
        pack(Point { x: 126, y: 270 }).unwrap()
    );
}
#[test]
fn screen_overflow_rejected_before_posting() {
    let request = input(
        json!({"action":"scroll","hwnd":1,"x":0,"y":0,"client_area":true,"scroll_amount":120}),
    );
    let geometry = Geometry {
        client: Point { x: 40000, y: 0 },
        ..Geometry::default()
    };
    assert!(plan::build(&request, geometry, State::default()).is_err());
}
