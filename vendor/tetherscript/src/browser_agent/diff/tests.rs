use super::{diff_html, DomDiffKind};

#[test]
fn summarizes_text_change() {
    let diff = diff_html("<p>old</p>", "<p>new</p>");

    assert_eq!(diff.entries[0].kind, DomDiffKind::TextChanged);
    assert_eq!(diff.entries[0].path, "/p[0]/#text[0]");
    assert_eq!(diff.entries[0].before.as_deref(), Some("old"));
    assert_eq!(diff.entries[0].after.as_deref(), Some("new"));
}

#[test]
fn summarizes_attribute_change() {
    let diff = diff_html(
        r#"<button class="a">Go</button>"#,
        r#"<button class="b">Go</button>"#,
    );

    assert_eq!(diff.entries[0].kind, DomDiffKind::AttributesChanged);
    assert_eq!(diff.entries[0].path, "/button[0]");
    assert_eq!(diff.entries[0].before.as_deref(), Some(r#"class="a""#));
    assert_eq!(diff.entries[0].after.as_deref(), Some(r#"class="b""#));
}

#[test]
fn summarizes_inserted_node() {
    let diff = diff_html("<p>A</p>", "<p>A</p><section>B</section>");

    assert_eq!(diff.entries[0].kind, DomDiffKind::Inserted);
    assert_eq!(diff.entries[0].path, "/section[1]");
    assert_eq!(diff.entries[0].after.as_deref(), Some("<section>"));
}

#[test]
fn summarizes_removed_node() {
    let diff = diff_html("<p>A</p><section>B</section>", "<p>A</p>");

    assert_eq!(diff.entries[0].kind, DomDiffKind::Removed);
    assert_eq!(diff.entries[0].path, "/section[1]");
    assert_eq!(diff.entries[0].before.as_deref(), Some("<section>"));
}
