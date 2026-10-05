//! Exercise prepared packing through projection assembly and the overflow clamp.
use super::{fixtures, select};
use crate::session::context::{
    incremental_clamp::clamp_and_recompute, incremental_insert::interleave,
};
use crate::session::helper::token::{estimate_request_tokens, estimate_tokens_for_messages};

#[test]
fn prepared_summary_survives_the_final_clamp() {
    let (index, _) = fixtures::prepared();
    let source = fixtures::transcript();
    let costs: Vec<usize> = source
        .iter()
        .map(|message| estimate_tokens_for_messages(std::slice::from_ref(message)))
        .collect();
    let budget = 100;
    let remaining = budget - estimate_request_tokens("", &source[2..], &[]);
    let mut keep = [false, false, true];
    let gaps = select(&index, &mut keep, &costs, vec![1, 0], remaining);
    let (mut messages, mut levels, mut origins) = interleave(&source, &keep, &gaps);
    let (dropped, tags) = clamp_and_recompute(
        &mut messages,
        &mut levels,
        &mut origins,
        "",
        &[],
        budget,
        3,
        &[(0, 2)],
    );
    assert_eq!(messages.len(), 2);
    assert_eq!(levels[0], crate::session::ResidencyLevel::Compressed);
    assert!(dropped.is_empty());
    assert!(!tags.contains(&"incremental_overflow_clamp"));
    assert!(estimate_request_tokens("", &messages, &[]) <= budget);
    assert_eq!(source.len(), 3, "canonical history is untouched");
}
