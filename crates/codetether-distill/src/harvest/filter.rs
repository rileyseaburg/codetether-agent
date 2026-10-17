//! Selection policy for accepted training records.
/// Filter criteria for harvesting training records.
///
/// # Examples
/// ```
/// use codetether_distill::harvest::HarvestFilter;
/// let filter = HarvestFilter::default();
/// assert_eq!(filter.max_records, 1000);
/// ```
pub struct HarvestFilter {
    pub min_quality_score: f32,
    pub only_passed_gates: bool,
    pub max_records: usize,
}

impl Default for HarvestFilter {
    fn default() -> Self {
        Self {
            min_quality_score: 0.8,
            only_passed_gates: true,
            max_records: 1000,
        }
    }
}
