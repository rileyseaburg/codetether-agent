#[test]
fn typescript_addition_has_syntax_spans() {
    let diff = concat!(
        "--- a/src/tabs.ts\n",
        "+++ b/src/tabs.ts\n",
        "@@ -0,0 +1,2 @@\n",
        "+export const tabs: readonly string[] = [\"build\"]\n",
        "+console.log(tabs)\n",
    );

    let lines = super::approval_diff::lines(diff);

    assert!(
        lines[3]
            .spans
            .iter()
            .skip(1)
            .any(|span| span.style.fg.is_some())
    );
    assert_eq!(lines[3].spans[0].content, "+");
}
