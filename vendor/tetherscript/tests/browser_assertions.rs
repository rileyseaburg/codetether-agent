use std::{any::Any, cell::RefCell, rc::Rc};

use tetherscript::{
    capability::Authority,
    interp::Interpreter,
    lexer::Lexer,
    parser::Parser,
    value::{Runtime, Value},
};

#[derive(Default)]
struct BrowserStub(RefCell<Vec<String>>);

impl Authority for BrowserStub {
    fn narrow(&self, _: &Value) -> Result<Rc<dyn Authority>, String> {
        Ok(Rc::new(Self::default()))
    }

    fn invoke(&self, _: &mut dyn Runtime, method: &str, _: &[Value]) -> Result<Value, String> {
        self.0.borrow_mut().push(method.into());
        match method {
            "console_errors" | "failed_requests" => Ok(Value::List(Rc::new(RefCell::new(vec![])))),
            "is_visible" | "is_enabled" => Ok(Value::Bool(true)),
            _ => Ok(Value::Nil),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

#[test]
fn assertion_helpers_call_browser_capability_methods() {
    let src = "fn main(){assert_selector(browser,\"#go\")? assert_text(browser,\"Ready\")? assert_no_console_errors(browser)? assert_no_failed_requests(browser)? assert_visible(browser,\"#go\")? assert_enabled(browser,\"#go\")? assert_route(browser,\"/ready\")? assert_react_component(browser,\"Checkout\")?}";
    let tokens = Lexer::new(src).tokenize().unwrap();
    let program = Parser::new(tokens).parse_program().unwrap();
    let auth = Rc::new(BrowserStub::default());
    let mut interp = Interpreter::new();

    interp.grant("browser", auth.clone());
    interp.run(&program).unwrap();

    assert_eq!(
        auth.0.borrow().join(","),
        "wait_for_selector,wait_for_text,console_errors,failed_requests,is_visible,is_enabled,wait_for_url,react.component_tree"
    );
}
