use crate::browser_js::eval_with_dom;
use crate::js::JsValue;

#[test]
fn insert_adjacent_element_supports_all_positions() {
    let result = eval_with_dom(
        "<main id='app'><p id='a'>A</p></main>",
        "let app=document.getElementById('app');let a=document.getElementById('a');\
         let i=document.createElement('i');i.textContent='I';\
         let b=document.createElement('b');b.textContent='B';\
         let em=document.createElement('em');em.textContent='E';\
         let s=document.createElement('strong');s.textContent='S';\
         a.insertAdjacentElement('beforebegin',i);\
         a.insertAdjacentElement('afterend',b);\
         app.insertAdjacentElement('afterbegin',em);\
         app.insertAdjacentElement('beforeend',s);\
         document.getElementById('app').innerHTML;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("<em>E</em><i>I</i><p id=\"a\">A</p><b>B</b><strong>S</strong>".into())
    );
}

#[test]
fn insert_adjacent_element_returns_element_or_null() {
    let result = eval_with_dom(
        "<main id='app'></main>",
        "let app=document.getElementById('app');let el=document.createElement('span');\
         el.textContent='R';let returned=app.insertAdjacentElement('beforeend',el);\
         let missing=app.insertAdjacentElement('beforeend','not-node');\
         returned.textContent+':'+(returned===el)+':'+missing\
         +':'+document.getElementById('app').innerHTML;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("R:true:null:<span>R</span>".into())
    );
}
