//! Preserve the on-disk schema independently of the runtime factory type.
use super::{AutoEmbedder, MemoryEntry, Store};
use codetether_vectordb::TextEmbedder;
use std::sync::Arc;

#[derive(Debug, Clone, Default)]
struct FactoryWithoutSerde;

#[async_trait::async_trait]
impl AutoEmbedder for FactoryWithoutSerde {
    async fn create() -> Option<Arc<dyn TextEmbedder>> {
        None
    }
}

#[test]
fn store_round_trip_keeps_only_legacy_entries_field() {
    let mut store = Store::<FactoryWithoutSerde>::default();
    store.add(MemoryEntry::new("persisted knowledge", vec!["test".into()]));
    let saved = serde_json::to_value(&store).expect("serialize without factory bounds");
    let fields = saved.as_object().expect("store object");
    assert_eq!(fields.len(), 1);
    assert!(fields.contains_key("entries"));
    let restored: Store<FactoryWithoutSerde> =
        serde_json::from_value(saved.clone()).expect("deserialize without factory bounds");
    assert_eq!(restored.stats().total_entries, 1);
    assert_eq!(serde_json::to_value(restored).unwrap(), saved);
}
