//! Verify stop markers split across UTF-8 token deltas never leak.
use super::Filter;
#[test]
fn split_stop_is_withheld_and_terminates() {
    let mut filter = Filter::new(vec!["<END>".into()]);
    let mut output = String::new();
    let mut emit = |text: &str| {
        output.push_str(text);
        Ok(())
    };
    assert!(filter.push("hello<EN", &mut emit).unwrap());
    assert!(!filter.push("D>hidden", &mut emit).unwrap());
    filter.finish(&mut emit).unwrap();
    assert_eq!(output, "hello");
}
#[test]
fn unmatched_partial_stop_flushes_at_eos() {
    let mut filter = Filter::new(vec!["世界".into()]);
    let mut output = String::new();
    let mut emit = |text: &str| {
        output.push_str(text);
        Ok(())
    };
    assert!(filter.push("hello世", &mut emit).unwrap());
    filter.finish(&mut emit).unwrap();
    assert_eq!(output, "hello世");
}
#[test]
fn earliest_complete_stop_wins() {
    let mut filter = Filter::new(vec!["stop".into(), "end".into()]);
    let mut output = String::new();
    let mut emit = |text: &str| {
        output.push_str(text);
        Ok(())
    };
    assert!(!filter.push("aendbstop", &mut emit).unwrap());
    assert_eq!(output, "a");
}
