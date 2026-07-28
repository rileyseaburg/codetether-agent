use super::*;

#[test]
fn reflected_attributes_read_initial_values() {
    let result = eval_with_dom(
        "<a id='a' name='home' href='/go' title='Link'></a><img id='i' src='pic.png' alt='Pic'><input id='q' type='search' placeholder='Find'>",
        "let a=document.getElementById('a');let i=document.getElementById('i');let q=document.getElementById('q');\
         a.name+'|'+a.href+'|'+a.title+'|'+i.src+'|'+i.alt+'|'+q.type+'|'+q.placeholder;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("home|/go|Link|pic.png|Pic|search|Find".into())
    );
}

#[test]
fn reflected_attribute_setters_update_fresh_nodes() {
    let result = eval_with_dom(
        "<button id='b' disabled></button><a id='a'></a><img id='i'><input id='q'>",
        "let a=document.getElementById('a');a.name='n2';a.href='/new';a.title='T2';\
         let i=document.getElementById('i');i.src='new.png';i.alt='Alt2';\
         let q=document.getElementById('q');q.type='email';q.placeholder='Email';q.disabled=true;\
         let first=document.getElementById('q');let firstState=first.disabled+':'+first.hasAttribute('disabled');\
         q.disabled=false;let fresh=document.getElementById('q');\
         let b=document.getElementById('b');b.disabled=false;let rb=document.getElementById('b');\
         document.getElementById('a').getAttribute('name')+'|'+document.getElementById('a').href+'|'\
         +document.getElementById('a').title+'|'+document.getElementById('i').src+'|'\
         +document.getElementById('i').alt+'|'+fresh.type+'|'+fresh.placeholder+'|'+firstState+'|'\
         +fresh.disabled+':'+fresh.hasAttribute('disabled')+'|'+rb.disabled+':'+rb.hasAttribute('disabled');",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String(
            "n2|/new|T2|new.png|Alt2|email|Email|true:true|false:false|false:false".into()
        )
    );
}
