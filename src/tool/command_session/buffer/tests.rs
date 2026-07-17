use super::Buffer;

#[test]
fn output_below_the_budget_is_unchanged() {
    let mut buffer = Buffer::new(16);
    buffer.push(b"compiler output");
    let (output, omitted) = buffer.finish();
    assert_eq!(output, "compiler output");
    assert_eq!(omitted, 0);
}

#[test]
fn oversized_output_keeps_stable_head_and_recent_tail() {
    let mut buffer = Buffer::new(10);
    buffer.push(b"012345");
    buffer.push(b"6789");
    buffer.push(b"ABCDEF");
    let (output, omitted) = buffer.finish();
    assert_eq!(output, "01234\n... 6 bytes omitted ...\nBCDEF");
    assert_eq!(omitted, 6);
}

#[test]
fn one_byte_budget_retains_the_latest_byte() {
    let mut buffer = Buffer::new(1);
    buffer.push(b"abc");
    let (output, omitted) = buffer.finish();
    assert_eq!(output, "\n... 2 bytes omitted ...\nc");
    assert_eq!(omitted, 2);
}
