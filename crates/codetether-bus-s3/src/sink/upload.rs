//! upload for the S3 bus adapter.
use super::*;
impl BusS3Sink {
    pub(super) async fn upload(&self, s3_key: String, jsonl: String, count: usize) -> Result<()> {
        let spool = s3_spool::SpoolFile::write(&s3_key, &jsonl).await?;
        let content = ObjectContent::from(jsonl.into_bytes());

        match self
            .client
            .put_object_content(&self.config.bucket, &s3_key, content)?
            .build()
            .send()
            .await
        {
            Ok(_) => {
                info!(
                    bucket = %self.config.bucket,
                    key = %s3_key,
                    records = count,
                    "Uploaded training records to S3"
                );
                spool.remove().await?;
            }
            Err(e) => {
                error!(
                    bucket = %self.config.bucket,
                    key = %s3_key,
                    error = %e,
                    path = %spool.path().display(),
                    "Failed to upload training records to S3; retained local spool file"
                );
                return Err(anyhow::anyhow!("S3 upload failed: {e}"));
            }
        }

        Ok(())
    }
}
