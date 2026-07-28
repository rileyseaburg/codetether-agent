use super::super::*;

#[test]
fn structured_clone_copies_plain_values() {
    let result = eval_with_dom(
        "<main></main>",
        "let s={name:'a',list:[1,{ok:true}],nil:null};\
         let c=structuredClone(s);\
         (c!==s)+':' +(c.list!==s.list)+':' +(c.list[1]!==s.list[1])+':' +c.list[1].ok+':' +(c.nil===null);",
    )
    .unwrap();

    assert_eq!(
        result.value,
        JsValue::String("true:true:true:true:true".into())
    );
}

#[test]
fn structured_clone_rejects_function_values() {
    let error = eval_error("let s={handler:function(){}}; structuredClone(s);");

    assert!(error.contains("structuredClone: cannot clone function value"));
}

#[test]
fn structured_clone_rejects_cycles() {
    let error = eval_error("let s={}; s.self=s; structuredClone(s);");

    assert!(error.contains("structuredClone: cyclic object values are not supported"));
}

fn eval_error(script: &str) -> String {
    match eval_with_dom("<main></main>", script) {
        Ok(_) => panic!("expected structuredClone to fail"),
        Err(error) => error,
    }
}
