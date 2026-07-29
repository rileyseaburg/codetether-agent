use super::{process_run, process_types};
use std::any::Any;
use std::rc::Rc;
use tetherscript::capability::Authority;
use tetherscript::value::{Runtime, Value};

pub struct ProcessAuthority {
    progress_id: Option<String>,
}

impl ProcessAuthority {
    pub fn new(progress_id: Option<String>) -> Rc<dyn Authority> {
        Rc::new(Self { progress_id })
    }
}

impl Authority for ProcessAuthority {
    fn narrow(&self, _params: &Value) -> Result<Rc<dyn Authority>, String> {
        Ok(Self::new(self.progress_id.clone()))
    }

    /// Dispatches an authority method.
    ///
    /// Per TetherScript's authority contract, the interpreter lifts a native
    /// `Ok(v)` into exactly one `Value::Result(Ok(v))` and a native `Err(e)`
    /// into `Value::Result(Err(e))`. Returning an already-wrapped
    /// `Value::Result` here would therefore produce `Result<Result<map>>`.
    fn invoke(&self, _rt: &mut dyn Runtime, method: &str, args: &[Value]) -> Result<Value, String> {
        match method {
            "run" => run(self.progress_id.as_deref(), args),
            _ => Err(format!("codetether_process: no method `{method}`")),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Runs a subprocess and returns the bare result map.
///
/// Both branches must yield the same arity so one plugin source works on every
/// execution path. The progress-reporting branch previously wrapped
/// [`process_run::run`]'s `Result` a second time, so `process_run(...)` returned
/// `Result<Result<map>>` under the `tetherscript_plugin` tool while
/// `tetherscript::system::process_run` returned `Result<map>` under
/// `tetherscript run`. Plugins then needed `??` on one path and `?` on the
/// other, and indexing the inner `Result` failed with
/// "cannot index result with str".
fn run(progress_id: Option<&str>, args: &[Value]) -> Result<Value, String> {
    match progress_id {
        Some(id) => process_run::run(id, args),
        None => process_types::unwrap_result(tetherscript::system::process_run(args)),
    }
}
