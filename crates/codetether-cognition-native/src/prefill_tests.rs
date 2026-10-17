//! Tests for chunked prefill sizing.

use super::{MAX_VEC_KERNEL_BATCH, chunks};

#[test]
fn cpu_prefills_in_one_pass() {
    assert_eq!(chunks(58, false), vec![58]);
}

#[test]
fn cuda_splits_long_prompts() {
    // The 58-token prompt that OOM'd an 8 GB card.
    let sizes = chunks(58, true);
    assert_eq!(sizes.iter().sum::<usize>(), 58, "must cover every token");
    assert!(
        sizes.iter().all(|n| *n <= MAX_VEC_KERNEL_BATCH),
        "every chunk must stay on the vector kernel: {sizes:?}"
    );
    assert_eq!(sizes, vec![8, 8, 8, 8, 8, 8, 8, 2]);
}

#[test]
fn exact_multiples_have_no_remainder_chunk() {
    assert_eq!(chunks(16, true), vec![8, 8]);
}

#[test]
fn short_prompts_are_untouched() {
    assert_eq!(chunks(8, true), vec![8]);
    assert_eq!(chunks(1, true), vec![1]);
}

#[test]
fn empty_prompt_yields_no_chunks() {
    assert!(chunks(0, true).is_empty());
    assert!(chunks(0, false).is_empty());
}
