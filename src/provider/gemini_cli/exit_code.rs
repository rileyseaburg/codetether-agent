//! Upstream Gemini CLI exit-code semantics.
//!
//! Documented in `docs/cli/headless.md` of google-gemini/gemini-cli.

/// Classification of one Gemini CLI process exit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExitKind {
    /// Exit code 0.
    Success,
    /// Exit code 1: general error or API failure.
    GeneralError,
    /// Exit code 42: invalid prompt or arguments.
    InputError,
    /// Exit code 53: turn limit exceeded.
    TurnLimitExceeded,
    /// Any other code, including signals.
    Unknown(i32),
}

/// Maps a raw process exit code to its upstream meaning.
pub fn classify(code: Option<i32>) -> ExitKind {
    match code {
        Some(0) => ExitKind::Success,
        Some(1) => ExitKind::GeneralError,
        Some(42) => ExitKind::InputError,
        Some(53) => ExitKind::TurnLimitExceeded,
        Some(other) => ExitKind::Unknown(other),
        None => ExitKind::Unknown(-1),
    }
}

/// Returns the operator-facing description for one exit classification.
pub fn describe(kind: ExitKind) -> &'static str {
    match kind {
        ExitKind::Success => "gemini exited successfully",
        ExitKind::GeneralError => "gemini reported a general error or API failure",
        ExitKind::InputError => "gemini rejected the prompt or arguments",
        ExitKind::TurnLimitExceeded => "gemini exceeded its turn limit",
        ExitKind::Unknown(_) => "gemini exited with an unrecognized status",
    }
}

#[cfg(test)]
#[path = "exit_code_tests.rs"]
mod tests;