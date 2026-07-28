use super::super::super::*;

#[test]
fn dom_constructors_image_defaults_are_deterministic() {
    let result = eval_with_dom(
        "<main></main>",
        "let img=Image();img.tagName+'|'+img.nodeName+'|'\
         +img.width+'|'+img.height+'|'+img.complete+'|'\
         +img.naturalWidth+'|'+img.naturalHeight+'|'\
         +img.decoding+'|'+img.loading+'|'+img.src;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("IMG|IMG|0|0|true|0|0|auto|eager|".into())
    );
}

#[test]
fn dom_constructors_image_size_and_src_reflect() {
    let result = eval_with_dom(
        "<main></main>",
        "let img=new Image(64,48);img.src='/asset.png';\
         img.width+'|'+img.height+'|'+img.naturalWidth+'|'\
         +img.naturalHeight+'|'+img.getAttribute('width')+'|'\
         +img.getAttribute('height')+'|'+img.getAttribute('src')+'|'+img.src;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("64|48|64|48|64|48|/asset.png|/asset.png".into())
    );
}

#[test]
fn dom_constructors_option_fields_and_select_append_work() {
    let result = eval_with_dom(
        "<select id='s'></select>",
        "let a=Option('Alpha','A',true,false);\
         let b=Option('Beta','B',false,true);\
         document.getElementById('s').appendChild(b);\
         a.textContent+'|'+a.value+'|'+a.defaultSelected+'|'+a.selected+'>'\
         +b.textContent+'|'+b.value+'|'+b.defaultSelected+'|'+b.selected+'>'\
         +document.getElementById('s').length+'|'+document.getElementById('s').value;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("Alpha|A|true|false>Beta|B|false|true>1|B".into())
    );
}
