use super::*;

#[test]
fn sibling_convenience_methods_mutate_serialized_dom() {
    let result = eval_with_dom(
        "<main id='app'><p id='a'>A</p><p id='b'>B</p></main>",
        "let app=document.getElementById('app'); let a=document.getElementById('a'); a.before('0'); a.after('1'); app.prepend('P'); app.append('Q'); let em=document.createElement('em'); em.textContent='Y'; document.getElementById('b').replaceWith('X', em); document.getElementById('app').textContent;",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("P0A1XYQ".into()));
    assert!(result.html.contains("<em>Y</em>"));
}

#[test]
fn insert_adjacent_html_and_text_support_common_positions() {
    let result = eval_with_dom(
        "<main id='app'><p id='a'>A</p></main>",
        "let app=document.getElementById('app'); let a=document.getElementById('a'); a.insertAdjacentHTML('beforebegin','<i>I</i>'); a.insertAdjacentText('afterend','T'); app.insertAdjacentHTML('afterbegin','<b>B</b>'); app.insertAdjacentText('beforeend','E'); let fresh=document.getElementById('app'); fresh.textContent + '|' + fresh.innerHTML;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("BIATE|<b>B</b><i>I</i><p id=\"a\">A</p>TE".into())
    );
}
