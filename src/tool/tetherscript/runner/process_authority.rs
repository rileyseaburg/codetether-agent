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

    fn invoke(&self, _rt: &mut dyn Runtime, method: &str, args: &[Value]) -> Result<Value, String> {
        match method {
            "run" => Ok(run(self.progress_id.as_deref(), args)),
            _ => Err(format!("codetether_process: no method `{method}`")),
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

fn run(progress_id: Option<&str>, args: &[Value]) -> Value {
    match progress_id {
        Some(id) => process_types::wrap(process_run::run(id, args)),
        None => tetherscript::system::process_run(args),
    }
}
