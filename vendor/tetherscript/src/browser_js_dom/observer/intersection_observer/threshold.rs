//! Intersection threshold crossing detection.

pub fn crossed(old: f64, new: f64, thresholds: &[f64]) -> bool {
    old < 0.0
        || thresholds
            .iter()
            .any(|&t| (old < t && new >= t) || (old >= t && new < t))
}
