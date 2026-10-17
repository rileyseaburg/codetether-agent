//! bucket for the S3 bus adapter.
use super::*;
impl BusS3Sink {
    /// Ensure the bucket exists, creating it if necessary
    pub async fn ensure_bucket(&self) -> Result<()> {
        match self
            .client
            .bucket_exists(&self.config.bucket)?
            .build()
            .send()
            .await
        {
            Ok(resp) if resp.exists() => {
                debug!(bucket = %self.config.bucket, "S3 bucket exists");
            }
            Ok(_) => {
                info!(bucket = %self.config.bucket, "Creating S3 bucket");
                match self
                    .client
                    .create_bucket(&self.config.bucket)?
                    .build()
                    .send()
                    .await
                {
                    Ok(_) => {}
                    Err(e) => {
                        let err_text = e.to_string();
                        if !err_text.contains("BucketAlreadyOwnedByYou")
                            && !err_text.contains("BucketAlreadyExists")
                        {
                            return Err(anyhow::anyhow!("Failed to create bucket: {err_text}"));
                        }
                        debug!(bucket = %self.config.bucket, "Bucket already exists");
                    }
                }
            }
            Err(e) => {
                debug!(error = %e, bucket = %self.config.bucket, "Bucket check returned error (may already exist)");
            }
        }
        Ok(())
    }
}
