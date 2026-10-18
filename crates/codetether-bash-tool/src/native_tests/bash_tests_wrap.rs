use super::super::codetether_wrapped_command;

#[test]
fn wraps_commands_with_codetether_function() {
    let wrapped = codetether_wrapped_command("codetether run 'hi'");
    assert!(wrapped.contains("codetether()"));
    assert!(wrapped.contains("CODETETHER_BIN"));
    assert!(wrapped.ends_with("codetether run 'hi'"));
}
