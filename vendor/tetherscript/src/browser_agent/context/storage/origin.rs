//! Origin normalization for DOM storage buckets.

pub(super) fn storage_origin(input: &str) -> String {
    super::super::indexed_db::indexed_db_origin(input)
}
