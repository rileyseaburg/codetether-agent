use std::rc::Rc;

use tetherscript::{
    browser_cap::BrowserAuthority,
    capability::Authority,
    value::{Runtime, Value},
};

struct NoopRuntime;

impl Runtime for NoopRuntime {
    fn invoke(&mut self, _: &Value, _: &[Value]) -> Result<Value, String> {
        Ok(Value::Nil)
    }
}

fn str_value(text: &str) -> Value {
    Value::Str(Rc::new(text.into()))
}

fn invoke(auth: &Rc<dyn Authority>, method: &str, args: &[Value]) -> Result<Value, String> {
    auth.invoke(&mut NoopRuntime, method, args)
}

#[test]
fn trace_exports_failed_live_actions_for_agent_debugging() {
    let auth = BrowserAuthority::new(
        "http://127.0.0.1:1/browser",
        Vec::new(),
        BrowserAuthority::all_scopes(),
    );

    let _ = invoke(&auth, "goto", &[str_value("http://app.test")]);
    let summary = invoke(&auth, "agent_summary", &[]).unwrap();
    let repro = invoke(&auth, "minimal_reproduction_script", &[]).unwrap();

    match summary {
        Value::Map(m) => {
            assert_eq!(m.borrow().get("action_count"), Some(&Value::Int(1)));
            assert_eq!(m.borrow().get("error_count"), Some(&Value::Int(1)));
        }
        other => panic!("expected summary map, got {}", other.type_name()),
    }
    assert!(format!("{repro}").contains("goto"));
}
