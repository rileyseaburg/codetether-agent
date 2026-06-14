pub const SOURCE: &str = r#"
fn process_run(command, args, stdin, timeout_ms) {
    return codetether_process.run(command, args, stdin, timeout_ms)
}
"#;

pub fn inject(source: String) -> String {
    format!("{SOURCE}{source}")
}
