use super::*;

#[test]
fn dom_parser_normalizes_html_documents() {
    let result = eval_with_dom("<main></main>", "let d=DOMParser().parseFromString('<p id=\"x\">Hi</p>', 'text/html'); d.documentElement.nodeName + ':' + d.head.nodeName + ':' + d.body.firstChild.id + ':' + d.body.textContent;").unwrap();
    assert_eq!(result.value, JsValue::String("html:head:x:Hi".into()));
}

#[test]
fn dom_parser_parses_svg_documents() {
    let script = "\
        let d=DOMParser().parseFromString('<svg><title>Logo</title><circle id=\"dot\" r=\"4\" /></svg>', 'image/svg+xml');\
        d.documentElement.nodeName + ':' + d.querySelector('circle').id + ':' + d.textContent;";
    let result = eval_with_dom("<main></main>", script).unwrap();
    assert_eq!(result.value, JsValue::String("svg:dot:Logo".into()));
}

#[test]
fn dom_parser_serializes_xml_documents() {
    let script = "\
        let a=DOMParser().parseFromString('<root><item>A&B</item></root>', 'application/xml');\
        let b=DOMParser().parseFromString('<note><to>Ada</to></note>', 'text/xml');\
        XMLSerializer().serializeToString(a) + '|' + a.querySelector('item').textContent + '|' + b.documentElement.nodeName + ':' + b.querySelector('to').textContent;";
    let result = eval_with_dom("<main></main>", script).unwrap();
    assert_eq!(
        result.value,
        JsValue::String("<root><item>A&amp;B</item></root>|A&B|note:Ada".into())
    );
}
