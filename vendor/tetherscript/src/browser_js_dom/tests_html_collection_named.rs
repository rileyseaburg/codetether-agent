use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn document_collections_support_named_item() {
    let result = eval_with_dom(
        "<form id='login'></form><form name='search'></form><img id='hero'>",
        "document.forms.namedItem('login').id+':'\
         +document.forms.namedItem('search').getAttribute('name')+':'\
         +document.images.namedItem('hero').id+':'\
         +(document.forms.namedItem('missing')===null);",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("login:search:hero:true".into())
    );
}

#[test]
fn html_collections_expose_named_properties() {
    let result = eval_with_dom(
        "<form id='signup'></form><form name='profile'></form>\
         <form id='item'></form><form id='0' name='zero'></form>",
        "document.forms.signup.id+':'\
         +document.forms.profile.getAttribute('name')+':'\
         +typeof document.forms.item+':'\
         +document.forms[0].id+':'+document.forms.zero.id;",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("signup:profile:function:signup:0".into())
    );
}

#[test]
fn get_elements_by_tag_collection_supports_named_access() {
    let result = eval_with_dom(
        "<span id='first'></span><span name='second'></span>\
         <span id='first' name='third'></span>",
        "let spans=document.getElementsByTagName('span');\
         spans.namedItem('first').id+':'\
         +spans.namedItem('second').getAttribute('name')+':'\
         +spans.third.id+':'+(spans.namedItem('missing')===null);",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("first:second:first:true".into())
    );
}
