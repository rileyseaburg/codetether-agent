use super::super::lines as guard_lines;

#[test]
fn comments_and_blanks_do_not_count() {
    let text = "\n// comment\n/* block\nstill */\nconst x = 1;\n";
    assert_eq!(guard_lines::code_lines(text), 1);
}
