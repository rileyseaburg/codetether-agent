use super::ExecutionProvenance;

pub fn provenance_trailers(provenance: &ExecutionProvenance) -> Vec<(&'static str, String)> {
    let mut trailers = required_trailers(provenance);
    super::trailer_optional::push_optional_trailers(&mut trailers, provenance);
    trailers
}

fn required_trailers(provenance: &ExecutionProvenance) -> Vec<(&'static str, String)> {
    vec![
        ("CodeTether-Provenance-ID", provenance.provenance_id.clone()),
        (
            "CodeTether-Origin",
            provenance.identity.origin.as_str().to_string(),
        ),
        (
            "CodeTether-Agent-Name",
            provenance.identity.agent_name.clone(),
        ),
    ]
}
