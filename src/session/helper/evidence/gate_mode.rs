#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GateMode {
    Off,
    Warn,
    Strict,
}

pub(crate) fn current() -> GateMode {
    match std::env::var("CODETETHER_SCOPE_GATE")
        .unwrap_or_else(|_| "warn".to_string())
        .to_ascii_lowercase()
        .as_str()
    {
        "off" => GateMode::Off,
        "strict" => GateMode::Strict,
        _ => GateMode::Warn,
    }
}
