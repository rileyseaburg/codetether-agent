//! Optional JavaScript stack suffix used by error display.
pub(super) fn stack_suffix(stack: Option<&str>) -> String {
    stack.map(|value| format!("\n{value}")).unwrap_or_default()
}
