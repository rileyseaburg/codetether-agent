//! Geolocation position values.

/// Deterministic geolocation coordinates.
///
/// # Examples
///
/// ```
/// use tetherscript::browser_agent::permissions::GeolocationPosition;
///
/// let pos = GeolocationPosition::new(51.5, -0.12, 8.0).unwrap();
/// assert_eq!(pos.accuracy, 8.0);
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GeolocationPosition {
    /// Latitude in degrees.
    pub latitude: f64,
    /// Longitude in degrees.
    pub longitude: f64,
    /// Accuracy radius in meters.
    pub accuracy: f64,
}

impl GeolocationPosition {
    /// Create and validate a deterministic geolocation position.
    pub fn new(latitude: f64, longitude: f64, accuracy: f64) -> Result<Self, String> {
        if !latitude.is_finite() || !(-90.0..=90.0).contains(&latitude) {
            return Err("geolocation latitude must be finite and between -90 and 90".into());
        }
        if !longitude.is_finite() || !(-180.0..=180.0).contains(&longitude) {
            return Err("geolocation longitude must be finite and between -180 and 180".into());
        }
        if !accuracy.is_finite() || accuracy < 0.0 {
            return Err("geolocation accuracy must be finite and non-negative".into());
        }
        Ok(Self {
            latitude,
            longitude,
            accuracy,
        })
    }
}
