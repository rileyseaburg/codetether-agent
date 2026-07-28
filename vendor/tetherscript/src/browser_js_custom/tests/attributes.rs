use super::super::super::*;

#[test]
fn observed_attribute_callback_receives_old_and_new_values() {
    let result = eval_with_dom(
        "<main id='app'></main>",
        "customElements.define('x-attr',{observedAttributes:['data-x'],\
         attributeChangedCallback:function(n,o,v){console.log(n+':'+o+':'+v);}});\
         let el=document.createElement('x-attr'); let live=document.getElementById('app').appendChild(el);\
         live.setAttribute('data-x','one'); live.setAttribute('title','skip');\
         live.removeAttribute('data-x'); 'ok';",
    )
    .unwrap();
    assert_eq!(result.console, vec!["data-x:null:one", "data-x:one:null"]);
}
