use super::super::super::*;

#[test]
fn dom_node_global_exposes_type_and_position_constants() {
    let result = eval_with_dom(
        "<main></main>",
        "typeof Node+'|'+Node.ELEMENT_NODE+'|'+Node.ATTRIBUTE_NODE+'|'\
         +Node.TEXT_NODE+'|'+Node.DOCUMENT_NODE+'|'\
         +Node.DOCUMENT_TYPE_NODE+'|'+Node.DOCUMENT_FRAGMENT_NODE+'|'\
         +Node.DOCUMENT_POSITION_DISCONNECTED+'|'\
         +Node.DOCUMENT_POSITION_FOLLOWING+'|'\
         +Node.DOCUMENT_POSITION_IMPLEMENTATION_SPECIFIC;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("function|1|2|3|9|10|11|1|4|32".into())
    );
}

#[test]
fn lightweight_dom_type_globals_are_probeable() {
    let result = eval_with_dom(
        "<main></main>",
        "typeof Element+'|'+typeof HTMLElement+'|'\
         +typeof Document+'|'+typeof DocumentFragment;",
    )
    .unwrap();
    assert_eq!(
        result.value,
        JsValue::String("function|function|function|function".into())
    );
}

#[test]
fn unsupported_dom_constructors_return_clear_errors() {
    let node_error = eval_error("new Node();");
    let element_error = eval_error("Element();");
    assert!(node_error.contains("Node constructor is not supported"));
    assert!(element_error.contains("Element constructor is not supported"));
}

fn eval_error(script: &str) -> String {
    match eval_with_dom("<main></main>", script) {
        Err(error) => error,
        Ok(_) => panic!("expected script to fail"),
    }
}
