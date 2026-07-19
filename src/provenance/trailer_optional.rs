use super::ExecutionProvenance;

pub fn push_optional_trailers(
    trailers: &mut Vec<(&'static str, String)>,
    provenance: &ExecutionProvenance,
) {
    super::trailer_forgejo::push_forgejo_trailers(trailers);
    super::trailer_identity::push_identity_trailers(trailers, provenance);
    super::trailer_run::push_run_trailers(trailers, provenance);
}
