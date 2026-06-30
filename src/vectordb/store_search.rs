//! Top-k cosine search over a [`VectorStore`].

use super::record::Record;
use super::similarity::cosine;
use super::store::VectorStore;
use super::vector::EmbeddingVector;
use serde::Serialize;
use serde::de::DeserializeOwned;

/// A scored search hit referencing a stored record.
#[derive(Debug, Clone)]
pub struct Hit<'a, P> {
    /// Cosine similarity in `[-1.0, 1.0]`.
    pub score: f32,
    /// The matched record.
    pub record: &'a Record<P>,
}

impl<P> VectorStore<P>
where
    P: Serialize + DeserializeOwned + Clone,
{
    /// Return the `top_k` records most similar to `query`, descending by score.
    ///
    /// Records whose similarity is `<= 0.0` are excluded.
    pub fn search(&self, query: &EmbeddingVector, top_k: usize) -> Vec<Hit<'_, P>> {
        let mut hits: Vec<Hit<'_, P>> = self
            .records()
            .iter()
            .map(|record| Hit {
                score: cosine(query, &record.vector),
                record,
            })
            .filter(|hit| hit.score > 0.0)
            .collect();

        hits.sort_by(|a, b| {
            b.score
                .partial_cmp(&a.score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        hits.truncate(top_k);
        hits
    }
}
