use super::{MAX_PER_RESPONSE, extract};

fn call(index: usize) -> String {
    format!(r#"<tool_call>{{"name":"read","arguments":{{"path":"{index}"}}}}</tool_call>"#)
}

#[test]
fn caps_large_batches_until_real_results_arrive() {
    let response = (0..114).map(call).collect::<String>();
    let (_, calls) = extract(&response);

    assert_eq!(calls.len(), MAX_PER_RESPONSE);
    assert!(calls[0].1.contains(r#""path":"0""#));
    assert!(calls[7].1.contains(r#""path":"7""#));
}

#[test]
fn suppresses_exact_duplicate_calls() {
    let response = format!("{}{}", call(3), call(3));
    let (_, calls) = extract(&response);

    assert_eq!(calls.len(), 1);
}

#[test]
fn preserves_html_entities_inside_argument_values() {
    let text = concat!(
        "&lt;tool-call&gt;",
        r#"{"name":"write","arguments":{"html":"&lt;div&gt;"}}"#,
        "&lt;/tool-call&gt;"
    );
    let (_, calls) = extract(text);
    assert!(calls[0].1.contains(r#""html":"&lt;div&gt;""#));
}
