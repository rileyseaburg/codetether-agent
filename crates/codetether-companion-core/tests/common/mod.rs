use codetether_companion_protocol::SessionInput;

pub fn input() -> SessionInput {
    SessionInput {
        model: "provider/vision".into(),
        prompt: "  Describe the screen  ".into(),
        interval_seconds: 30,
    }
}
pub const NOW: i64 = 1_767_225_600_000;
