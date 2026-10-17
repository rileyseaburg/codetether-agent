use super::*;

/// Task-scoped publication and lookup implemented by [SubTaskStoreHandle].
///
/// # Implementors
/// [SubTaskStoreHandle] binds each published result to a producer task ID.
pub trait ResultStoreContext {
    /// Publish a result to the shared store.
    ///
    /// # Errors
    /// Returns an error if the value cannot be serialized.
    fn publish_result(
        &self,
        key: impl Into<String> + Send,
        value: impl Serialize + Send,
        tags: Vec<String>,
    ) -> impl std::future::Future<Output = Result<SharedResult>> + Send;

    /// Get a result from the shared store
    fn get_result(
        &self,
        key: &str,
    ) -> impl std::future::Future<Output = Option<SharedResult>> + Send;

    /// Get a typed result from the shared store.
    ///
    /// # Errors
    /// Returns an error if the key is absent or decoding fails.
    fn get_result_typed<T: for<'de> Deserialize<'de>>(
        &self,
        key: &str,
    ) -> impl std::future::Future<Output = Result<T>> + Send;
}
