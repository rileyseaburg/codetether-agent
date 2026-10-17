//! access for the S3 bus adapter.
use super::*;
impl BusS3Sink {
    #[allow(dead_code)]
    /// Get the bucket name
    pub fn bucket(&self) -> &str {
        &self.config.bucket
    }

    #[allow(dead_code)]
    /// Get the prefix
    pub fn prefix(&self) -> &str {
        &self.config.prefix
    }
}
