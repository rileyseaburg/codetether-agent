//! Execute a ranged object GET and retain typed backend/read failures.
use crate::PointerHandle;
use codetether_context_policy::fault::Fault;
use minio::s3::{client::MinioClient, types::S3Api};
pub(crate) async fn bytes(
    client: MinioClient,
    handle: &PointerHandle,
    offset: Option<u64>,
    length: Option<u64>,
) -> Result<Vec<u8>, Fault> {
    let body = client
        .get_object(&handle.bucket, &handle.key)
        .map_err(|e| Fault::BackendError {
            reason: format!("GET s3://{}/{} failed: {e}", handle.bucket, handle.key),
        })?
        .offset(offset)
        .length(length)
        .build()
        .send()
        .await
        .map_err(|e| Fault::BackendError {
            reason: format!("GET s3://{}/{} failed: {e}", handle.bucket, handle.key),
        })?
        .content()
        .map_err(|e| Fault::BackendError {
            reason: format!("read body for {} failed: {e}", handle.key),
        })?
        .to_segmented_bytes()
        .await
        .map_err(|e| Fault::BackendError {
            reason: format!("read body for {} failed: {e}", handle.key),
        })?
        .to_bytes();
    if body.is_empty() {
        return Err(Fault::NoMatch);
    }
    Ok(body.to_vec())
}
