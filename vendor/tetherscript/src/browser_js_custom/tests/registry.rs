use super::super::super::*;

#[test]
fn custom_elements_define_get_when_defined_and_constructor_work() {
    let result = eval_with_dom(
        "<main id='app'></main>",
        "let seen=''; let ready=customElements.whenDefined('x-card'); \
         ready.then(function(def){ seen='ready:' + (def === customElements.get('x-card')); }); \
         customElements.define('x-card',{constructor:function(){ this.setAttribute('data-made','yes'); }}); \
         let el=document.createElement('x-card'); document.getElementById('app').appendChild(el); \
         seen + ':' + document.querySelector('x-card').getAttribute('data-made');",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("ready:true:yes".into()));
}
