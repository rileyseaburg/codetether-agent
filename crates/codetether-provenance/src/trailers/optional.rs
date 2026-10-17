//! Ordered assembly of optional provenance trailers.
use crate::ExecutionProvenance;

pub fn push_optional_trailers(
    trailers: &mut Vec<(&'static str, String)>,
    provenance: &ExecutionProvenance,
) {
    super::forgejo::push_forgejo_trailers(trailers);
    super::identity::push_identity_trailers(trailers, provenance);
    super::run::push_run_trailers(trailers, provenance);
}
