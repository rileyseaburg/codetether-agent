use codetether_companion_protocol::SessionInput;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Boundary {
    pub age: i64,
    pub status: u16,
}

#[derive(Deserialize)]
pub struct Fixture {
    pub now: i64,
    pub input: SessionInput,
    pub pairing: Vec<Boundary>,
    pub session: Vec<Boundary>,
    pub capacity: usize,
    pub attempts: usize,
    pub window_ms: i64,
}

pub fn fixture() -> Fixture {
    serde_json::from_str(include_str!("../../fixtures/lifecycle.json")).unwrap()
}
pub fn status<T>(result: Result<T, codetether_companion_core::Error>) -> u16 {
    result.map_or_else(|error| error.status(), |_| 200)
}
