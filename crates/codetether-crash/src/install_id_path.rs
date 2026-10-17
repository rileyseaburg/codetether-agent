//! Crash reporting install id path.
use super::*;
pub(super) fn install_id_path() -> PathBuf {
    codetether_data_dir().join("telemetry").join("install_id")
}
