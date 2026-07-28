use super::*;

#[test]
fn attribute_names_and_toggle_attribute_are_available() {
    let result = eval_with_dom(
        "<div id='x' class='c'></div>",
        "let el=document.getElementById('x'); let before=el.getAttributeNames().join(','); let a=el.toggleAttribute('hidden'); let b=el.toggleAttribute('hidden', false); let c=el.toggleAttribute('data-on', true); before + ':' + a + b + c + ':' + el.hasAttribute('hidden') + ':' + el.getAttribute('data-on') + ':' + el.getAttributeNames().join(',');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("class,id:truefalsetrue:false::class,data-on,id".into())
    );
}

#[test]
fn inline_style_methods_sync_the_style_attribute() {
    let result = eval_with_dom(
        "<div id='x' style='color: red'></div>",
        "let el=document.getElementById('x'); let s=el.style; let before=s.getPropertyValue('color') + ':' + s.cssText; s.setProperty('display','none'); let removed=s.removeProperty('color'); s.cssText='margin-top: 2px;'; before + '|' + removed + '|' + s.getPropertyValue('display') + '|' + s.cssText + '|' + el.getAttribute('style');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("red:color: red;|red||margin-top: 2px;|margin-top: 2px;".into())
    );
}
