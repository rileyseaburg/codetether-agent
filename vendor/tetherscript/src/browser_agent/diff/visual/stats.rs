//! Diff statistics.

#[derive(Debug, Clone, PartialEq)]
pub struct DiffStats {
    pub total_pixels: usize,
    pub changed_pixels: usize,
    pub max_color_distance: u32,
    pub diff_percentage: f64,
}
