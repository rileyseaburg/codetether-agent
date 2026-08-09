//! Randomized transcript shapes checked against the Bedrock pairing
//! invariant, so unpaired-toolUse 400s are caught locally rather than by the
//! service.

use super::invariant::violation;
use crate::provider::bedrock::convert::convert_messages;

#[path = "fuzz/generate.rs"]
mod generate;

#[test]
fn randomized_transcripts_always_convert_to_paired_bedrock_messages() {
    for seed in 0..500u64 {
        let input = generate::transcript(seed);
        let (_, messages) = convert_messages(&input);
        if let Some(problem) = violation(&messages) {
            panic!(
                "seed {seed}: {problem}\ninput roles: {:?}\nbody: {}",
                input.iter().map(|m| m.role).collect::<Vec<_>>(),
                serde_json::json!(messages)
            );
        }
    }
}
