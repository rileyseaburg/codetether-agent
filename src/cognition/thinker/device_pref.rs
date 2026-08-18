//! Device preference for local inference, independent of any backend crate.

/// Device preference for local Candle inference.
///
/// Parsing is available in every build so configuration stays uniform even
/// when the `candle` feature is not compiled in.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::cognition::CandleDevicePreference;
/// assert_eq!(CandleDevicePreference::from_env("cpu"), CandleDevicePreference::Cpu);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CandleDevicePreference {
    /// Prefer CUDA when compiled and available, else CPU.
    #[default]
    Auto,
    /// Force CPU execution.
    Cpu,
    /// Require a CUDA device.
    Cuda,
}

impl CandleDevicePreference {
    /// Parse a device preference from an env-var string.
    ///
    /// Unknown values fall back to [`CandleDevicePreference::Auto`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use codetether_agent::cognition::CandleDevicePreference;
    /// assert_eq!(CandleDevicePreference::from_env("cuda"), CandleDevicePreference::Cuda);
    /// assert_eq!(CandleDevicePreference::from_env("weird"), CandleDevicePreference::Auto);
    /// ```
    pub fn from_env(value: &str) -> Self {
        match value.trim().to_ascii_lowercase().as_str() {
            "cpu" => Self::Cpu,
            "cuda" | "gpu" => Self::Cuda,
            _ => Self::Auto,
        }
    }
}
