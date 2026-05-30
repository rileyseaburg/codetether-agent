//! Display impl for TaskCheckpoint.

use super::TaskCheckpoint;

impl std::fmt::Display for TaskCheckpoint {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
