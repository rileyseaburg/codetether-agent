use super::super::super::*;

#[test]
fn existing_custom_element_gets_connected_and_disconnected_callbacks() {
    let result = eval_with_dom(
        "<main><x-log id='one'></x-log></main>",
        "customElements.define('x-log',{constructor:function(){console.log('ctor:'+this.id);},\
         connectedCallback:function(){console.log('connect:'+this.id);},\
         disconnectedCallback:function(){console.log('disconnect:'+this.id);}});\
         document.getElementById('one').remove(); 'done';",
    )
    .unwrap();
    assert_eq!(result.value, JsValue::String("done".into()));
    assert_eq!(
        result.console,
        vec!["ctor:one", "connect:one", "disconnect:one"]
    );
}
