//! Tests for [`ProviderEmbedder`](super::provider::ProviderEmbedder).

use super::*;
use std::sync::Arc;

#[path = "provider_test_fixture.rs"]
mod fixture;
use fixture::FixedEmbedProvider;

#[tokio::test]
async fn provider_embedder_normalizes_remote_vectors() {
    let embedder = ProviderEmbedder::new(Arc::new(FixedEmbedProvider), "fixed-model");
    let out = embedder
        .embed_batch(&["a".into(), "b".into()])
        .await
        .unwrap();
    assert_eq!(out.len(), 2);
    let norm: f32 = out[0].as_slice().iter().map(|x| x * x).sum::<f32>().sqrt();
    assert!(
        (norm - 1.0).abs() < 1e-6,
        "remote vectors must be L2-normalized"
    );
}

#[tokio::test]
async fn provider_embedder_empty_input_is_empty() {
    let embedder = ProviderEmbedder::new(Arc::new(FixedEmbedProvider), "fixed-model");
    assert!(embedder.embed_batch(&[]).await.unwrap().is_empty());
}
