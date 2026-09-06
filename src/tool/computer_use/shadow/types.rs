//! Portable logical state and queued-message plan records.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub(super) struct Point {
    pub x: i32,
    pub y: i32,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize)]
pub(super) struct State {
    pub pointer_client: Option<Point>,
    pub buttons: u16,
    pub pending_key: Option<(u16, isize)>,
}
#[derive(Clone, Copy, Debug, Default)]
pub(super) struct Geometry {
    pub outer: Point,
    pub client: Point,
}
#[derive(Clone, Debug)]
pub(super) struct Event {
    pub message: u32,
    pub wparam: usize,
    pub lparam: isize,
    pub delay_ms: u64,
    pub after: State,
}
#[derive(Default)]
pub(super) struct Plan {
    pub events: Vec<Event>,
    pub state: State,
}
impl Plan {
    pub fn push(&mut self, message: u32, wparam: usize, lparam: isize, delay_ms: u64) {
        self.events.push(Event {
            message,
            wparam,
            lparam,
            delay_ms,
            after: self.state,
        });
    }
}
