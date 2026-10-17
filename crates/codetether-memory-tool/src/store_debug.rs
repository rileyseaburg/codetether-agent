//! Preserve the store's debug view without exposing its factory marker.
use super::Store;
impl<B> std::fmt::Debug for Store<B> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MemoryStore")
            .field("entries", &self.entries)
            .field("embedder", &self.embedder)
            .finish()
    }
}
