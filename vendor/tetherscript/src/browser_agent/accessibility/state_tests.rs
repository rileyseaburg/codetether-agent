use crate::browser_agent::BrowserPage;

#[test]
fn snapshot_extracts_common_native_and_aria_states() {
    let page = BrowserPage::from_html(
        "mem://a11y",
        "<button disabled aria-expanded='true' aria-pressed='mixed'>Save</button>\
         <input type='checkbox' checked><option selected>One</option>\
         <div role='checkbox' aria-checked='false'></div>",
    );
    let roots = page.accessibility_snapshot().roots;

    assert!(roots[0].states.disabled);
    assert_eq!(roots[0].states.expanded.as_deref(), Some("true"));
    assert_eq!(roots[0].states.pressed.as_deref(), Some("mixed"));
    assert_eq!(roots[1].states.checked.as_deref(), Some("true"));
    assert_eq!(roots[2].states.selected.as_deref(), Some("true"));
    assert_eq!(roots[3].states.checked.as_deref(), Some("false"));
}
