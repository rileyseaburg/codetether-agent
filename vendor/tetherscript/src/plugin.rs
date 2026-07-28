//! Embeddable TetherScript plugin runtime.
//!
//! A Rust host creates a [`PluginHost`], grants project-specific capabilities,
//! loads `.tether` or legacy `.tether` source, then calls named hook functions. Plugins run with the
//! sandboxed built-in set by default; all host authority must be explicit.

use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::capability::{with_audit_capture, Authority, CapabilityAuditEvent};
use crate::interp::{with_step_budget, Interpreter, Unwind};
use crate::lexer::Lexer;
use crate::output;
use crate::parser::Parser;
use crate::value::{Runtime, Slot, Value};

pub const DEFAULT_PLUGIN_STEP_BUDGET: u64 = 200_000;
pub const DEFAULT_PLUGIN_OUTPUT_LIMIT: usize = 64 * 1024;

pub struct PluginHost {
    grants: Vec<(String, Rc<dyn Authority>)>,
    step_budget: u64,
    output_limit: usize,
    audit_capabilities: bool,
}

pub struct LoadedPlugin {
    name: String,
    interp: Interpreter,
    load_stdout: String,
    step_budget: u64,
    output_limit: usize,
    audit_capabilities: bool,
}

#[derive(Debug, Clone)]
pub struct PluginCall {
    pub value: Value,
    pub stdout: String,
    pub audit: Vec<CapabilityAuditEvent>,
}

#[derive(Debug)]
pub enum PluginError {
    Io {
        path: PathBuf,
        message: String,
    },
    Lex {
        plugin: String,
        line: usize,
        col: usize,
        message: String,
    },
    Parse {
        plugin: String,
        line: usize,
        col: usize,
        message: String,
    },
    Load {
        plugin: String,
        message: String,
        stdout: String,
    },
    MissingHook {
        plugin: String,
        hook: String,
    },
    Hook {
        plugin: String,
        hook: String,
        message: String,
        stdout: String,
    },
}

impl PluginHost {
    pub fn new() -> Self {
        Self {
            grants: Vec::new(),
            step_budget: DEFAULT_PLUGIN_STEP_BUDGET,
            output_limit: DEFAULT_PLUGIN_OUTPUT_LIMIT,
            audit_capabilities: false,
        }
    }

    pub fn with_step_budget(mut self, budget: u64) -> Self {
        self.step_budget = budget;
        self
    }

    pub fn with_output_limit(mut self, limit: usize) -> Self {
        self.output_limit = limit;
        self
    }

    pub fn with_capability_audit(mut self, enabled: bool) -> Self {
        self.audit_capabilities = enabled;
        self
    }

    pub fn grant(&mut self, name: impl Into<String>, authority: Rc<dyn Authority>) -> &mut Self {
        self.grants.push((name.into(), authority));
        self
    }

    pub fn load_file(&self, path: impl AsRef<Path>) -> Result<LoadedPlugin, PluginError> {
        let path = path.as_ref();
        let source = fs::read_to_string(path).map_err(|e| PluginError::Io {
            path: path.to_path_buf(),
            message: e.to_string(),
        })?;
        self.load_source(path.display().to_string(), &source)
    }

    pub fn load_dir(&self, dir: impl AsRef<Path>) -> Result<Vec<LoadedPlugin>, PluginError> {
        let dir = dir.as_ref();
        let mut paths = Vec::new();
        for entry in fs::read_dir(dir).map_err(|e| PluginError::Io {
            path: dir.to_path_buf(),
            message: e.to_string(),
        })? {
            let entry = entry.map_err(|e| PluginError::Io {
                path: dir.to_path_buf(),
                message: e.to_string(),
            })?;
            let path = entry.path();
            if is_tetherscript_source(&path) {
                paths.push(path);
            }
        }
        paths.sort();

        let mut plugins = Vec::with_capacity(paths.len());
        for path in paths {
            plugins.push(self.load_file(path)?);
        }
        Ok(plugins)
    }

    pub fn load_source(
        &self,
        name: impl Into<String>,
        source: &str,
    ) -> Result<LoadedPlugin, PluginError> {
        let name = name.into();
        let tokens = Lexer::new(source)
            .tokenize()
            .map_err(|e| PluginError::Lex {
                plugin: name.clone(),
                line: e.line,
                col: e.col,
                message: e.msg,
            })?;
        let program = Parser::new(tokens)
            .parse_program()
            .map_err(|e| PluginError::Parse {
                plugin: name.clone(),
                line: e.line,
                col: e.col,
                message: e.msg,
            })?;

        let mut interp = Interpreter::new_sandboxed();
        for (grant_name, authority) in &self.grants {
            interp.grant(grant_name, authority.clone());
        }

        let (stdout, (_audit, result)) = output::with_capture(self.output_limit, || {
            self.with_optional_audit(|| {
                with_step_budget(self.step_budget, || interp.run_repl(&program))
            })
        });
        match result {
            Ok(_) => Ok(LoadedPlugin {
                name,
                interp,
                load_stdout: stdout,
                step_budget: self.step_budget,
                output_limit: self.output_limit,
                audit_capabilities: self.audit_capabilities,
            }),
            Err(message) => Err(PluginError::Load {
                plugin: name,
                message,
                stdout,
            }),
        }
    }

    fn with_optional_audit<F, R>(&self, f: F) -> (Vec<CapabilityAuditEvent>, R)
    where
        F: FnOnce() -> R,
    {
        if self.audit_capabilities {
            with_audit_capture(f)
        } else {
            (Vec::new(), f())
        }
    }
}

impl Default for PluginHost {
    fn default() -> Self {
        Self::new()
    }
}

fn is_tetherscript_source(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|ext| ext.to_str()),
        Some("tether" | "kl")
    )
}

impl LoadedPlugin {
    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn load_stdout(&self) -> &str {
        &self.load_stdout
    }

    pub fn has_hook(&self, hook: &str) -> bool {
        matches!(
            self.interp.globals.borrow().slots.get(hook),
            Some(Slot::Live {
                value: Value::Fn(_) | Value::VmFn(_) | Value::Native(_),
                ..
            })
        )
    }

    pub fn metadata(&mut self) -> Result<Option<PluginCall>, PluginError> {
        if self.has_hook("plugin") {
            self.call("plugin", &[]).map(Some)
        } else {
            Ok(None)
        }
    }

    pub fn call(&mut self, hook: &str, args: &[Value]) -> Result<PluginCall, PluginError> {
        let callee = {
            let globals = self.interp.globals.borrow();
            match globals.get(hook) {
                Ok(value) => value,
                Err(message) if message.starts_with("undefined variable") => {
                    return Err(PluginError::MissingHook {
                        plugin: self.name.clone(),
                        hook: hook.to_string(),
                    });
                }
                Err(message) => {
                    return Err(PluginError::Hook {
                        plugin: self.name.clone(),
                        hook: hook.to_string(),
                        message,
                        stdout: String::new(),
                    });
                }
            }
        };

        let (stdout, (audit, result)) = output::with_capture(self.output_limit, || {
            if self.audit_capabilities {
                with_audit_capture(|| {
                    with_step_budget(self.step_budget, || self.interp.call(&callee, args))
                })
            } else {
                (
                    Vec::new(),
                    with_step_budget(self.step_budget, || self.interp.call(&callee, args)),
                )
            }
        });
        match result {
            Ok(value) => Ok(PluginCall {
                value,
                stdout,
                audit,
            }),
            Err(err) => Err(PluginError::Hook {
                plugin: self.name.clone(),
                hook: hook.to_string(),
                message: unwind_message(err),
                stdout,
            }),
        }
    }
}

fn unwind_message(err: Unwind) -> String {
    match err {
        Unwind::Error(message) | Unwind::Panic(message) | Unwind::TryErr(message) => message,
        Unwind::Return(_) => "`return` unwound out of plugin hook".into(),
    }
}

impl fmt::Display for PluginError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PluginError::Io { path, message } => {
                write!(f, "{}: {}", path.display(), message)
            }
            PluginError::Lex {
                plugin,
                line,
                col,
                message,
            } => {
                write!(f, "{}: lex error at {}:{}: {}", plugin, line, col, message)
            }
            PluginError::Parse {
                plugin,
                line,
                col,
                message,
            } => {
                write!(
                    f,
                    "{}: parse error at {}:{}: {}",
                    plugin, line, col, message
                )
            }
            PluginError::Load {
                plugin, message, ..
            } => {
                write!(f, "{}: load failed: {}", plugin, message)
            }
            PluginError::MissingHook { plugin, hook } => {
                write!(f, "{}: missing hook `{}`", plugin, hook)
            }
            PluginError::Hook {
                plugin,
                hook,
                message,
                ..
            } => {
                write!(f, "{}: hook `{}` failed: {}", plugin, hook, message)
            }
        }
    }
}

impl Error for PluginError {}

/// Capability used by TetherScript's own plugin CLI path. It is intentionally small:
/// it lets a TetherScript plugin introspect and validate TetherScript source, not mutate the
/// host process.
pub struct TetherScriptAuthority;

impl TetherScriptAuthority {
    #[allow(clippy::new_ret_no_self)]
    pub fn new() -> Rc<dyn Authority> {
        Rc::new(Self)
    }
}

impl Authority for TetherScriptAuthority {
    fn narrow(&self, _params: &Value) -> Result<Rc<dyn Authority>, String> {
        Ok(Self::new())
    }

    fn invoke(&self, _rt: &mut dyn Runtime, method: &str, args: &[Value]) -> Result<Value, String> {
        match (method, args) {
            ("version", []) => Ok(Value::Str(Rc::new(env!("CARGO_PKG_VERSION").into()))),
            ("language", []) => Ok(Value::Str(Rc::new("TetherScript".into()))),
            ("diagnose", [Value::Str(source)])
            | ("validate_source", [Value::Str(source)]) => Ok(diagnose_source(source)),
            ("describe", []) => Ok(tetherscript_description()),
            (name, _) => Err(format!(
                "tetherscript: no method `{}` (have: version, language, diagnose, validate_source, describe)",
                name
            )),
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

fn tetherscript_description() -> Value {
    let mut map = HashMap::new();
    map.insert(
        "language".into(),
        Value::Str(Rc::new("TetherScript".into())),
    );
    map.insert(
        "version".into(),
        Value::Str(Rc::new(env!("CARGO_PKG_VERSION").into())),
    );
    map.insert(
        "plugin_api".into(),
        Value::Str(Rc::new("source hooks over explicit capabilities".into())),
    );
    map.insert(
        "host_methods".into(),
        Value::List(Rc::new(RefCell::new(vec![
            Value::Str(Rc::new("version".into())),
            Value::Str(Rc::new("language".into())),
            Value::Str(Rc::new("diagnose".into())),
            Value::Str(Rc::new("validate_source".into())),
            Value::Str(Rc::new("describe".into())),
        ]))),
    );
    Value::Map(Rc::new(RefCell::new(map)))
}

fn diagnose_source(source: &str) -> Value {
    match Lexer::new(source).tokenize() {
        Err(e) => diagnostic(false, "lex", e.msg, e.line, e.col),
        Ok(tokens) => match Parser::new(tokens).parse_program() {
            Ok(_) => diagnostic(true, "parse", String::new(), 0, 0),
            Err(e) => diagnostic(false, "parse", e.msg, e.line, e.col),
        },
    }
}

fn diagnostic(ok: bool, stage: &str, message: String, line: usize, col: usize) -> Value {
    let mut map = HashMap::new();
    map.insert("ok".into(), Value::Bool(ok));
    map.insert("stage".into(), Value::Str(Rc::new(stage.into())));
    map.insert("message".into(), Value::Str(Rc::new(message)));
    map.insert("line".into(), Value::Int(line as i64));
    map.insert("col".into(), Value::Int(col as i64));
    Value::Map(Rc::new(RefCell::new(map)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::value::ResultValue;

    #[test]
    fn plugin_can_call_host_capability() {
        let source = r#"
fn plugin() {
    let m = map()
    m.name = "self-validator"
    return m
}

fn validate() {
    let d = tetherscript.diagnose("fn main() { println(\"ok\") }")?
    if d.ok {
        return Ok("valid")
    }
    return Err(d.message)
}
"#;
        let mut host = PluginHost::new();
        host.grant("tetherscript", TetherScriptAuthority::new());
        let mut plugin = host.load_source("self-validator", source).unwrap();

        assert!(plugin.has_hook("plugin"));
        assert!(plugin.has_hook("validate"));

        let metadata = plugin.metadata().unwrap().unwrap();
        match metadata.value {
            Value::Map(map) => {
                assert_eq!(
                    map.borrow().get("name"),
                    Some(&Value::Str(Rc::new("self-validator".into())))
                );
            }
            other => panic!("expected plugin metadata map, got {:?}", other),
        }

        let call = plugin.call("validate", &[]).unwrap();
        match call.value {
            Value::Result(result) => match result.as_ref() {
                ResultValue::Ok(Value::Str(s)) => assert_eq!(s.as_ref(), "valid"),
                other => panic!(
                    "expected Ok(\"valid\"), got {:?}",
                    Value::Result(Rc::new(other.clone()))
                ),
            },
            other => panic!("expected result, got {:?}", other),
        }
    }

    #[test]
    fn missing_hook_is_reported() {
        let host = PluginHost::new();
        let mut plugin = host.load_source("empty", "let x = 1").unwrap();
        let err = plugin.call("not_here", &[]).unwrap_err();
        assert!(matches!(err, PluginError::MissingHook { .. }));
    }

    #[test]
    fn plugin_host_can_audit_capability_calls() {
        let source = r#"
fn validate() {
    return tetherscript.version()
}
"#;
        let mut host = PluginHost::new().with_capability_audit(true);
        host.grant("tetherscript", TetherScriptAuthority::new());
        let mut plugin = host.load_source("audited", source).unwrap();

        let call = plugin.call("validate", &[]).unwrap();

        assert_eq!(call.audit.len(), 1);
        assert_eq!(call.audit[0].kind, "tetherscript");
        assert_eq!(call.audit[0].method, "version");
        assert!(call.audit[0].allowed);

        match call.value {
            Value::Result(result) => assert!(matches!(result.as_ref(), ResultValue::Ok(_))),
            other => panic!("expected result, got {:?}", other),
        }
    }
}
