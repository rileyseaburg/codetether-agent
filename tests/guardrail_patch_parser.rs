//! Focused contracts for multi-file patches used by approval preflight.

#[path = "../src/tool/patch/hunk_builder.rs"]
mod hunk_builder;
#[path = "../src/tool/patch/parser.rs"]
mod parser;
#[path = "../src/tool/patch/types.rs"]
mod types;

#[test]
fn multi_file_headers_do_not_corrupt_or_retarget_hunks() {
    let patch = "--- a/one.rs\n+++ b/one.rs\n@@ -1 +1 @@\n-old one\n+new one\n\
                 --- a/two.rs\n+++ b/two.rs\n@@ -1 +1 @@\n-old two\n+new two\n";
    let hunks = parser::parse_patch(patch);
    assert_eq!(hunks.len(), 2);
    assert_eq!(hunks[0].file, "one.rs");
    assert_eq!(hunks[0].old_lines, ["old one"]);
    assert_eq!(hunks[0].new_lines, ["new one"]);
    assert_eq!(hunks[1].file, "two.rs");
    assert_eq!(hunks[1].old_lines, ["old two"]);
}

#[test]
fn new_file_header_stays_out_of_the_previous_file() {
    let patch = "--- a/old.rs\n+++ b/old.rs\n@@ -1 +1 @@\n-old\n+new\n\
                 --- /dev/null\n+++ b/created.rs\n@@ -0,0 +1 @@\n+created\n";
    let hunks = parser::parse_patch(patch);
    assert_eq!(hunks[0].file, "old.rs");
    assert_eq!(hunks[0].old_lines, ["old"]);
    assert_eq!(hunks[1].file, "created.rs");
    assert_eq!(hunks[1].start_line, 0);
    assert!(hunks[1].old_lines.is_empty());
}

#[test]
fn multiple_hunks_keep_the_same_file() {
    let patch = "--- a/one.rs\n+++ b/one.rs\n@@ -1 +1 @@\n-a\n+b\n@@ -3 +3 @@\n-c\n+d\n";
    assert!(
        parser::parse_patch(patch)
            .iter()
            .all(|hunk| hunk.file == "one.rs")
    );
}
