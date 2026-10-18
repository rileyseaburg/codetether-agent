//! Environment-selected final-answer evidence enforcement mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Policy for unsupported final-answer verification claims.
/// # Examples
/// ```
/// let mode = codetether_evidence_core::gate_mode::GateMode::Off;
/// assert_eq!(codetether_evidence_core::verification_gate::apply(mode, "answer"), "answer");
/// ```
pub enum GateMode {
    /// Leave all claims unchanged.
    Off,
    /// Append a warning about unsupported claims.
    Warn,
    /// Withhold unsupported verification lines.
    Strict,
}

/// Read the scope-gate mode from the environment.
/// # Arguments
/// Reads `CODETETHER_SCOPE_GATE`; no arguments are required.
/// # Returns
/// Off, Strict or the default Warn mode.
/// # Examples
/// ```
/// let _mode = codetether_evidence_core::gate_mode::current();
/// ```
pub fn current() -> GateMode {
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
