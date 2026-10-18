//! Read a compact persisted recall projection.
/// Read the indexed projection for the requested session identity.
/// # Arguments
/// * `id` - Durable session identity.
/// # Returns
/// Its indexed evidence when available and readable.
/// # Examples
/// `read::<S>(id).await` does not load the full transcript.
pub async fn read<S: crate::Session>(id: &str) -> Option<codetether_recall_index::IndexedSession> {
    codetether_recall_store::read(id, &crate::Host::<S>::default()).await
}
