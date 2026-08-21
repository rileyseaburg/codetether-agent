//! Provider-safe serialization of floating-point request values.

/// Round a Rust `f32` temperature before encoding it as a JSON number.
///
/// Some OpenRouter upstreams reject binary floating-point artifacts such as
/// `0.699999988079071`, even though they accept the intended value `0.7`.
pub(super) fn temperature(value: f32) -> f64 {
    const DECIMAL_PLACES: f64 = 1_000.0;
    (f64::from(value) * DECIMAL_PLACES).round() / DECIMAL_PLACES
}

#[cfg(test)]
#[path = "request_number_tests.rs"]
mod tests;
