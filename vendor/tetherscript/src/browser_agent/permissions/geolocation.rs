//! Geolocation success and error emulation.

use super::GeolocationPosition;

/// Browser geolocation error codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeolocationErrorCode {
    /// Permission denied.
    PermissionDenied,
    /// Position unavailable.
    PositionUnavailable,
    /// Request timed out.
    Timeout,
}

/// Deterministic geolocation error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct GeolocationError {
    /// Browser-style error code.
    pub code: GeolocationErrorCode,
    /// Human-readable error message.
    pub message: String,
}

/// Current geolocation emulation outcome.
#[derive(Clone, Debug, Default, PartialEq)]
pub enum GeolocationEmulation {
    /// No position is configured.
    #[default]
    Unavailable,
    /// Return a position to successful callers.
    Position(GeolocationPosition),
    /// Return a configured error to failed callers.
    Error(GeolocationError),
}

impl GeolocationErrorCode {
    pub(crate) fn number(self) -> u8 {
        match self {
            Self::PermissionDenied => 1,
            Self::PositionUnavailable => 2,
            Self::Timeout => 3,
        }
    }
}

impl GeolocationError {
    /// Create a deterministic geolocation error.
    pub fn new(code: GeolocationErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}
