use super::*;

#[test]
fn style_indexes_initial_inline_declarations() {
    let result = eval_with_dom(
        "<div id='x' style='color: red; margin-top: 2px'></div>",
        "let s=document.getElementById('x').style; s.length + ':' + s[0] + ':' + s[1] + ':' + s.item(0) + ':' + s.item(1) + ':' + s.item(2);",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("2:color:margin-top:color:margin-top:".into())
    );
}

#[test]
fn style_indexes_refresh_after_set_and_remove() {
    let result = eval_with_dom(
        "<div id='x' style='color: red'></div>",
        "let s=document.getElementById('x').style; s.setProperty('display','none'); let a=s.length + ':' + s[0] + ':' + s[1]; let old=s.removeProperty('color'); a + '|' + old + ':' + s.length + ':' + s[0] + ':' + s.item(1) + ':' + s[1];",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("2:color:display|red:1:display::undefined".into())
    );
}

#[test]
fn style_indexes_refresh_after_css_text_assignment() {
    let result = eval_with_dom(
        "<div id='x' style='color: red'></div>",
        "let el=document.getElementById('x'); let s=el.style; s.cssText='z-index: 1; color: blue;'; s.length + ':' + s[0] + ':' + s[1] + ':' + s.cssText + '|' + el.getAttribute('style');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("2:z-index:color:z-index: 1; color: blue;|z-index: 1; color: blue;".into())
    );
}
