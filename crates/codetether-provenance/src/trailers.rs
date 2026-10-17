//! Ordered Git trailers for execution, identity, and signed principal metadata.
//!
//! [provenance_trailers] emits required fields before optional identity and run fields.
mod forgejo;
mod identity;
mod optional;
mod run;
use super::ExecutionProvenance;

/// Collect required and available optional trailers in stable emission order.
///
/// # Arguments
/// * `provenance` - Execution record supplying trailer values.
///
/// # Returns
/// Label/value pairs, with required provenance fields before optional identity and run fields.
///
/// # Examples
/// ```
/// use codetether_provenance::{ExecutionProvenance, provenance_trailers};
/// let record = ExecutionProvenance::for_session("session", "worker");
/// assert_eq!(provenance_trailers(&record)[0].0, "CodeTether-Provenance-ID");
/// ```
pub fn provenance_trailers(provenance: &ExecutionProvenance) -> Vec<(&'static str, String)> {
    let mut trailers = required_trailers(provenance);
    optional::push_optional_trailers(&mut trailers, provenance);
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
