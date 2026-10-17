//! QualityCheckResult benchmark data.
use super::*;

/// Quality check result
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// use codetether_benchmark::*;
/// let result = QualityCheckResult { name: String::new(), passed: false, output: None, };
/// let _ = result;
/// ```
pub struct QualityCheckResult {
    /// Check name (typecheck, test, lint, build)
    pub name: String,

    /// Whether it passed
    pub passed: bool,

    /// Output/error message if failed
    pub output: Option<String>,
}
