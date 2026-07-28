//! Small self-contained JavaScript engine.
//!
//! This module intentionally uses only the Rust standard library. It is not a
//! web-compatible ECMAScript implementation yet, but it provides a real lexer,
//! parser, lexical environments, user functions, native functions, control flow,
//! arithmetic/comparison/logical operators, arrays, objects, and property access.
//! It is meant to be the JavaScript execution core that the experimental browser
//! module can grow around without introducing external crates.

use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

use crate::value::Value;

thread_local! {
    static ARRAY_PROPS: RefCell<HashMap<usize, HashMap<String, JsValue>>> =
        RefCell::new(HashMap::new());
}

#[path = "js_array_like.rs"]
mod js_array_like;
#[path = "js_array_prototype.rs"]
mod js_array_prototype;
#[path = "js_await.rs"]
mod js_await;
#[path = "js_budget.rs"]
mod js_budget;
#[path = "js_console.rs"]
mod js_console;
#[path = "js_date.rs"]
mod js_date;
#[path = "js_json.rs"]
mod js_json;
#[path = "js_math.rs"]
mod js_math;
#[path = "js_module_syntax.rs"]
mod js_module_syntax;
#[path = "js_number_globals.rs"]
mod js_number_globals;
#[path = "js_primitive_prototype.rs"]
mod js_primitive_prototype;
#[path = "js_prototypes.rs"]
mod js_prototypes;
#[path = "js_regex_runtime.rs"]
mod js_regex_runtime;
#[path = "js_string.rs"]
mod js_string;
#[path = "js_string_prototype.rs"]
mod js_string_prototype;
#[path = "js_trace.rs"]
mod js_trace;
#[path = "js_uri.rs"]
mod js_uri;
#[path = "js_regex_rewrite.rs"]
mod regex_rewrite;
#[path = "js_syntax_preflight.rs"]
mod syntax_preflight;
pub(crate) use js_await::with_drain as with_await_drain;
pub(crate) use syntax_preflight::reject as reject_unsupported_syntax;

type NativeCallback = dyn Fn(&[JsValue]) -> Result<JsValue, String>;

#[derive(Clone)]
pub enum JsValue {
    Undefined,
    Null,
    Bool(bool),
    Number(f64),
    String(String),
    Symbol(String),
    Array(Rc<RefCell<Vec<JsValue>>>),
    Object(Rc<RefCell<HashMap<String, JsValue>>>),
    Function(Rc<JsFunction>),
    BoundFunction(Rc<BoundFunction>),
    Class(Rc<JsClass>),
    Native(Rc<NativeFunction>),
}

#[derive(Clone)]
pub struct JsFunction {
    name: Option<String>,
    params: Vec<String>,
    body: Vec<Stmt>,
    env: EnvRef,
    superclass: Option<JsValue>,
    properties: RefCell<HashMap<String, JsValue>>,
}

#[derive(Clone)]
pub struct BoundFunction {
    pub function: JsValue,
    pub this_value: JsValue,
}

#[derive(Clone)]
pub struct JsClass {
    name: Option<String>,
    superclass: Option<JsValue>,
    constructor: Option<ClassMethod>,
    methods: Vec<ClassMethod>,
    static_methods: Vec<ClassMethod>,
    env: EnvRef,
    properties: RefCell<HashMap<String, JsValue>>,
}

#[derive(Clone)]
struct ClassMethod {
    name: String,
    params: Vec<String>,
    body: Vec<Stmt>,
}

pub struct NativeFunction {
    name: String,
    arity: Option<usize>,
    func: Box<NativeCallback>,
    properties: RefCell<HashMap<String, JsValue>>,
}

impl NativeFunction {
    pub fn new(
        name: impl Into<String>,
        arity: Option<usize>,
        func: impl Fn(&[JsValue]) -> Result<JsValue, String> + 'static,
    ) -> Self {
        Self {
            name: name.into(),
            arity,
            func: Box::new(func),
            properties: RefCell::new(HashMap::new()),
        }
    }

    pub fn with_property(mut self, name: impl Into<String>, value: JsValue) -> Self {
        self.properties.get_mut().insert(name.into(), value);
        self
    }

    fn property(&self, name: &str) -> Option<JsValue> {
        self.properties.borrow().get(name).cloned()
    }
}

impl fmt::Debug for JsValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display())
    }
}

impl PartialEq for JsValue {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (JsValue::Undefined, JsValue::Undefined) => true,
            (JsValue::Null, JsValue::Null) => true,
            (JsValue::Bool(a), JsValue::Bool(b)) => a == b,
            (JsValue::Number(a), JsValue::Number(b)) => (*a - *b).abs() < f64::EPSILON,
            (JsValue::String(a), JsValue::String(b)) => a == b,
            (JsValue::Symbol(a), JsValue::Symbol(b)) => a == b,
            (JsValue::Array(a), JsValue::Array(b)) => Rc::ptr_eq(a, b),
            (JsValue::Object(a), JsValue::Object(b)) => Rc::ptr_eq(a, b),
            (JsValue::Function(a), JsValue::Function(b)) => Rc::ptr_eq(a, b),
            (JsValue::BoundFunction(a), JsValue::BoundFunction(b)) => Rc::ptr_eq(a, b),
            (JsValue::Class(a), JsValue::Class(b)) => Rc::ptr_eq(a, b),
            (JsValue::Native(a), JsValue::Native(b)) => Rc::ptr_eq(a, b),
            _ => false,
        }
    }
}

impl JsValue {
    pub fn truthy(&self) -> bool {
        match self {
            JsValue::Undefined | JsValue::Null => false,
            JsValue::Bool(b) => *b,
            JsValue::Number(n) => *n != 0.0 && !n.is_nan(),
            JsValue::String(s) => !s.is_empty(),
            JsValue::Symbol(_) => true,
            JsValue::Array(_)
            | JsValue::Object(_)
            | JsValue::Function(_)
            | JsValue::BoundFunction(_)
            | JsValue::Class(_)
            | JsValue::Native(_) => true,
        }
    }

    pub fn display(&self) -> String {
        match self {
            JsValue::Undefined => "undefined".into(),
            JsValue::Null => "null".into(),
            JsValue::Bool(b) => b.to_string(),
            JsValue::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    n.to_string()
                }
            }
            JsValue::String(s) => s.clone(),
            JsValue::Symbol(s) => s.clone(),
            JsValue::Array(items) => items
                .borrow()
                .iter()
                .map(|v| v.display())
                .collect::<Vec<_>>()
                .join(","),
            JsValue::Object(obj) => object_display(&obj.borrow()),
            JsValue::Function(fun) => {
                format!("function {}", fun.name.as_deref().unwrap_or("<anonymous>"))
            }
            JsValue::BoundFunction(fun) => format!("bound {}", fun.function.display()),
            JsValue::Class(class) => format!("class {}", class.name.as_deref().unwrap_or("")),
            JsValue::Native(fun) => format!("function {}", fun.name),
        }
    }

    fn number(&self) -> f64 {
        match self {
            JsValue::Undefined => f64::NAN,
            JsValue::Null => 0.0,
            JsValue::Bool(b) => {
                if *b {
                    1.0
                } else {
                    0.0
                }
            }
            JsValue::Number(n) => *n,
            JsValue::String(s) if s.trim().is_empty() => 0.0,
            JsValue::String(s) => s.trim().parse().unwrap_or(f64::NAN),
            JsValue::Symbol(_) => f64::NAN,
            _ => f64::NAN,
        }
    }
}

pub fn js_to_tether(value: &JsValue) -> Value {
    match value {
        JsValue::Undefined | JsValue::Null => Value::Nil,
        JsValue::Bool(b) => Value::Bool(*b),
        JsValue::Number(n) if n.fract() == 0.0 => Value::Int(*n as i64),
        JsValue::Number(n) => Value::Float(*n),
        JsValue::String(s) => Value::Str(Rc::new(s.clone())),
        JsValue::Symbol(s) => Value::Str(Rc::new(s.clone())),
        JsValue::Array(items) => Value::List(Rc::new(RefCell::new(
            items.borrow().iter().map(js_to_tether).collect(),
        ))),
        JsValue::Object(obj) => {
            let map = obj
                .borrow()
                .iter()
                .map(|(k, v)| (k.clone(), js_to_tether(v)))
                .collect();
            Value::Map(Rc::new(RefCell::new(map)))
        }
        JsValue::Function(_)
        | JsValue::BoundFunction(_)
        | JsValue::Class(_)
        | JsValue::Native(_) => Value::Str(Rc::new(value.display())),
    }
}

pub fn eval_to_value(args: &[Value]) -> Result<Value, String> {
    if args.len() != 1 {
        return Err(format!("js_eval: expected 1 arg, got {}", args.len()));
    }
    let Value::Str(source) = &args[0] else {
        return Err(format!(
            "js_eval: expected str, got {}",
            args[0].type_name()
        ));
    };
    eval(source).map(|v| js_to_tether(&v))
}

pub fn eval(source: &str) -> Result<JsValue, String> {
    JsEngine::new().eval(source)
}

pub struct JsEngine {
    globals: EnvRef,
    last_console: Rc<RefCell<Vec<String>>>,
}

impl JsEngine {
    pub fn with_globals(globals: HashMap<String, JsValue>) -> Self {
        let engine = Self::new();
        {
            let mut env = engine.globals.borrow_mut();
            for (name, value) in globals {
                env.define(&name, value);
            }
        }
        engine
    }

    pub fn set_global(&mut self, name: &str, value: JsValue) {
        self.globals.borrow_mut().define(name, value);
    }

    pub fn new() -> Self {
        let globals = Env::new(None);
        let last_console = Rc::new(RefCell::new(Vec::new()));
        install_globals(&globals, last_console.clone());
        Self {
            globals,
            last_console,
        }
    }

    pub fn eval(&mut self, source: &str) -> Result<JsValue, String> {
        self.eval_inner(source)
    }

    pub fn eval_with_budget(&mut self, source: &str, budget: u64) -> Result<JsValue, String> {
        js_budget::with(budget, || self.eval_inner(source))
    }

    fn eval_inner(&mut self, source: &str) -> Result<JsValue, String> {
        js_trace::mark("syntax preflight start");
        syntax_preflight::reject(source)?;
        js_trace::mark("syntax preflight complete");
        js_trace::mark("regex rewrite start");
        let source = regex_rewrite::rewrite(source);
        js_trace::mark("regex rewrite complete");
        let source = js_module_syntax::strip_exports(&source);
        js_trace::mark("lex start");
        let tokens = Lexer::new(&source).tokenize()?;
        js_trace::mark("lex complete");
        js_trace::mark("parse start");
        let program = Parser::new(tokens).parse_program()?;
        js_trace::mark("parse complete");
        js_trace::mark("execute start");
        match execute_block(&program, self.globals.clone())? {
            Flow::Value(v) => Ok(v),
            Flow::Return(v) => Ok(v),
            Flow::Break(_) | Flow::Continue(_) => Err("break/continue outside loop".into()),
        }
    }

    pub fn console_output(&self) -> Vec<String> {
        self.last_console.borrow().clone()
    }
}

impl Default for JsEngine {
    fn default() -> Self {
        Self::new()
    }
}

fn install_globals(env: &EnvRef, console_log: Rc<RefCell<Vec<String>>>) {
    js_prototypes::reset();
    env.borrow_mut().define("undefined", JsValue::Undefined);
    env.borrow_mut().define("NaN", JsValue::Number(f64::NAN));
    env.borrow_mut()
        .define("Infinity", JsValue::Number(f64::INFINITY));

    js_console::install(env, console_log);
    js_math::install(env);
    js_json::install(env);
    js_number_globals::install(env);
    js_uri::install(env);

    let number_prototype = js_prototypes::register("Number", js_prototypes::empty());
    js_primitive_prototype::install_number();
    env.borrow_mut().define(
        "Number",
        JsValue::Native(Rc::new(
            NativeFunction::new("Number", Some(1), |args| {
                Ok(JsValue::Number(
                    args.first().unwrap_or(&JsValue::Undefined).number(),
                ))
            })
            .with_property("prototype", number_prototype),
        )),
    );
    env.borrow_mut().define("String", js_string::constructor());
    js_string_prototype::install();
    let boolean_prototype = js_prototypes::register("Boolean", js_prototypes::empty());
    js_primitive_prototype::install_boolean();
    env.borrow_mut().define(
        "Boolean",
        JsValue::Native(Rc::new(
            NativeFunction::new("Boolean", Some(1), |args| {
                Ok(JsValue::Bool(
                    args.first().unwrap_or(&JsValue::Undefined).truthy(),
                ))
            })
            .with_property("prototype", boolean_prototype),
        )),
    );
    env.borrow_mut()
        .define("Function", function_constructor(env.clone()));
    env.borrow_mut().define("Object", object_constructor());
    env.borrow_mut().define("Array", array_constructor());
    env.borrow_mut().define("Date", js_date::constructor());
    env.borrow_mut()
        .define("Set", collection_constructor("Set"));
    env.borrow_mut()
        .define("WeakSet", collection_constructor("WeakSet"));
    env.borrow_mut()
        .define("Map", collection_constructor("Map"));
    env.borrow_mut()
        .define("WeakMap", collection_constructor("WeakMap"));
    env.borrow_mut().define("Symbol", symbol_constructor());
    for name in [
        "Error",
        "TypeError",
        "RangeError",
        "ReferenceError",
        "SyntaxError",
        "URIError",
        "EvalError",
    ] {
        env.borrow_mut().define(name, error_constructor(name));
    }
    env.borrow_mut().define("eval", eval_function(env.clone()));
    env.borrow_mut().define("import", dynamic_import_function());
    env.borrow_mut().define(
        "__regex",
        JsValue::Native(Rc::new(NativeFunction::new("__regex", Some(2), |args| {
            Ok(regex_value(args[0].display(), args[1].display()))
        }))),
    );
    env.borrow_mut().define("RegExp", regex_constructor());

    for (name, func) in array_global_functions() {
        env.borrow_mut().define(name, func);
    }

    // ── Promise ──────────────────────────────────────────────────────────
    // Promise(executor) — returns a thenable object with synchronous resolution.
    // Promise.resolve / Promise.reject are also separate globals for older callers.
    env.borrow_mut().define(
        "Promise",
        JsValue::Native(Rc::new(NativeFunction::new("Promise", Some(1), |args| {
            let executor = args.first().cloned().unwrap_or(JsValue::Undefined);
            make_promise(executor)
        }))),
    );
    env.borrow_mut().define(
        "Promise_resolve",
        JsValue::Native(Rc::new(NativeFunction::new(
            "Promise_resolve",
            Some(1),
            promise_resolve,
        ))),
    );
    env.borrow_mut().define(
        "Promise_reject",
        JsValue::Native(Rc::new(NativeFunction::new(
            "Promise_reject",
            Some(1),
            promise_reject,
        ))),
    );
}

fn symbol_constructor() -> JsValue {
    let mut prototype = HashMap::new();
    prototype.insert(
        "valueOf".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Symbol.prototype.valueOf",
            Some(1),
            symbol_value_of,
        ))),
    );
    let prototype =
        js_prototypes::register("Symbol", JsValue::Object(Rc::new(RefCell::new(prototype))));
    JsValue::Native(Rc::new(
        NativeFunction::new("Symbol", None, |args| {
            let description = args.first().map(JsValue::display).unwrap_or_default();
            Ok(JsValue::Symbol(format!("Symbol({description})")))
        })
        .with_property(
            "for",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Symbol.for",
                Some(1),
                |args| {
                    Ok(JsValue::Symbol(format!(
                        "Symbol.for({})",
                        args[0].display()
                    )))
                },
            ))),
        )
        .with_property("iterator", JsValue::Symbol("Symbol.iterator".into()))
        .with_property("toStringTag", JsValue::Symbol("Symbol.toStringTag".into()))
        .with_property("prototype", prototype),
    ))
}

fn symbol_value_of(args: &[JsValue]) -> Result<JsValue, String> {
    match &args[0] {
        JsValue::Symbol(_) => Ok(args[0].clone()),
        _ => Err("TypeError: Symbol.prototype.valueOf requires Symbol".into()),
    }
}

fn error_constructor(name: &'static str) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(name, None, move |args| {
        let message = args.first().map(JsValue::display).unwrap_or_default();
        Ok(native_error_object(name, &message))
    })))
}

fn native_error_object(name: &str, message: &str) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("__error_name".into(), JsValue::String(name.into()));
    obj.insert("name".into(), JsValue::String(name.into()));
    obj.insert("message".into(), JsValue::String(message.into()));
    obj.insert("stack".into(), JsValue::String(name.into()));
    obj.insert("constructor".into(), constructor_name_object(name));
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn constructor_name_object(name: &str) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("name".into(), JsValue::String(name.into()));
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn object_display(obj: &HashMap<String, JsValue>) -> String {
    match obj.get("__error_name") {
        Some(JsValue::String(name)) => {
            let message = match obj.get("message") {
                Some(JsValue::String(message)) => message.as_str(),
                _ => "",
            };
            error_text(name, message)
        }
        _ if matches!(obj.get("message"), Some(JsValue::String(_))) => {
            let name = match obj.get("name") {
                Some(JsValue::String(name)) => name.as_str(),
                _ => "Error",
            };
            let Some(JsValue::String(message)) = obj.get("message") else {
                return "[object Object]".into();
            };
            error_text(name, message)
        }
        _ => "[object Object]".into(),
    }
}

fn error_text(name: &str, message: &str) -> String {
    if message.is_empty() {
        name.into()
    } else {
        format!("{name}: {message}")
    }
}

fn function_constructor(env: EnvRef) -> JsValue {
    let prototype = js_prototypes::register(
        "Function",
        JsValue::Native(Rc::new(
            NativeFunction::new("Function.prototype", None, |_| Ok(JsValue::Undefined))
                .with_property(
                    "toString",
                    JsValue::Native(Rc::new(NativeFunction::new(
                        "Function.prototype.toString",
                        None,
                        |args| {
                            Ok(JsValue::String(function_source(
                                args.first().unwrap_or(&JsValue::Undefined),
                            )))
                        },
                    ))),
                )
                .with_property("constructor", JsValue::String("Function".into())),
        )),
    );
    JsValue::Native(Rc::new(
        NativeFunction::new("Function", None, move |args| {
            dynamic_function_value(env.clone(), args)
        })
        .with_property("prototype", prototype),
    ))
}

fn function_source(value: &JsValue) -> String {
    match value {
        JsValue::Function(fun) => {
            format!("function {}() {{}}", fun.name.as_deref().unwrap_or(""))
        }
        JsValue::Native(native) => {
            let name = native.name.rsplit('.').next().unwrap_or(&native.name);
            format!("function {name}() {{ [native code] }}")
        }
        JsValue::BoundFunction(_) => "function bound() { [native code] }".into(),
        JsValue::Class(class) => format!("class {} {{}}", class.name.as_deref().unwrap_or("")),
        _ => "function () { [native code] }".into(),
    }
}

fn dynamic_function_value(env: EnvRef, args: &[JsValue]) -> Result<JsValue, String> {
    let body = args.last().map(JsValue::display).unwrap_or_default();
    let params = args
        .get(..args.len().saturating_sub(1))
        .unwrap_or(&[])
        .iter()
        .map(JsValue::display)
        .collect::<Vec<_>>()
        .join(",");
    let source = format!("function __dynamic_function({params}){{{body}}} __dynamic_function;");
    let source = regex_rewrite::rewrite(&source);
    let tokens = Lexer::new(&source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    match execute_block(&program, env)? {
        Flow::Value(value) | Flow::Return(value) => Ok(value),
        Flow::Break(_) | Flow::Continue(_) => Err("break/continue outside dynamic function".into()),
    }
}

fn eval_function(env: EnvRef) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new("eval", Some(1), move |args| {
        eval_source_in_env(
            &args.first().unwrap_or(&JsValue::Undefined).display(),
            env.clone(),
        )
    })))
}

fn eval_source_in_env(source: &str, env: EnvRef) -> Result<JsValue, String> {
    syntax_preflight::reject(source)?;
    let source = regex_rewrite::rewrite(source);
    let tokens = Lexer::new(&source).tokenize()?;
    let program = Parser::new(tokens).parse_program()?;
    match execute_block(&program, env)? {
        Flow::Value(value) | Flow::Return(value) => Ok(value),
        Flow::Break(_) | Flow::Continue(_) => Err("break/continue outside eval".into()),
    }
}

fn dynamic_import_function() -> JsValue {
    let mut meta = HashMap::new();
    meta.insert("url".into(), JsValue::String(String::new()));
    JsValue::Native(Rc::new(
        NativeFunction::new("import", Some(1), |_| {
            let module = JsValue::Object(Rc::new(RefCell::new(HashMap::new())));
            promise_resolve(&[module])
        })
        .with_property("meta", JsValue::Object(Rc::new(RefCell::new(meta)))),
    ))
}

fn regex_constructor() -> JsValue {
    let mut prototype = HashMap::new();
    prototype.insert(
        "exec".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "RegExp.prototype.exec",
            Some(2),
            regex_prototype_exec,
        ))),
    );
    let prototype =
        js_prototypes::register("RegExp", JsValue::Object(Rc::new(RefCell::new(prototype))));
    JsValue::Native(Rc::new(
        NativeFunction::new("RegExp", None, |args| {
            Ok(regex_value(
                args.first().map_or(String::new(), JsValue::display),
                args.get(1).map_or(String::new(), JsValue::display),
            ))
        })
        .with_property("prototype", prototype),
    ))
}

fn regex_prototype_exec(args: &[JsValue]) -> Result<JsValue, String> {
    let Some((pattern, flags)) = regex_parts(&args[0]) else {
        return Ok(JsValue::Null);
    };
    regex_exec(&pattern, &flags, &args[1].display())
}

fn regex_value(pattern: String, flags: String) -> JsValue {
    let mut obj = HashMap::new();
    let test_pattern = pattern.clone();
    let test_flags = flags.clone();
    let exec_pattern = pattern.clone();
    let exec_flags = flags.clone();
    obj.insert("__regex_pattern".into(), JsValue::String(pattern));
    obj.insert("__regex_flags".into(), JsValue::String(flags));
    obj.insert(
        "Symbol.toStringTag".into(),
        JsValue::String("RegExp".into()),
    );
    if let Some(prototype) = js_prototypes::get("RegExp") {
        obj.insert("__proto__".into(), prototype);
    }
    obj.insert(
        "test".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "RegExp.test",
            Some(1),
            move |args| {
                Ok(JsValue::Bool(
                    js_regex_runtime::exec(&args[0].display(), &test_pattern, &test_flags)
                        .is_some(),
                ))
            },
        ))),
    );
    obj.insert(
        "exec".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "RegExp.exec",
            Some(1),
            move |args| regex_exec(&exec_pattern, &exec_flags, &args[0].display()),
        ))),
    );
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn regex_exec(pattern: &str, flags: &str, text: &str) -> Result<JsValue, String> {
    let Some((start, end, captures)) = js_regex_runtime::exec(text, pattern, flags) else {
        return Ok(JsValue::Null);
    };
    let mut values = vec![JsValue::String(text[start..end].into())];
    values.extend(
        captures
            .into_iter()
            .map(|capture| capture.map(JsValue::String).unwrap_or(JsValue::Undefined)),
    );
    let array = Rc::new(RefCell::new(values));
    set_array_extra_property(&array, "index", JsValue::Number(start as f64));
    set_array_extra_property(&array, "input", JsValue::String(text.into()));
    Ok(JsValue::Array(array))
}

fn object_constructor() -> JsValue {
    let mut prototype = HashMap::new();
    prototype.insert(
        "hasOwnProperty".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Object.prototype.hasOwnProperty",
            None,
            |args| {
                Ok(JsValue::Bool(args.get(1).is_some_and(|key| {
                    has_own_property(&args[0], &key.display())
                })))
            },
        ))),
    );
    prototype.insert(
        "propertyIsEnumerable".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Object.prototype.propertyIsEnumerable",
            None,
            |args| {
                Ok(JsValue::Bool(args.get(1).is_some_and(|key| {
                    has_own_property(&args[0], &key.display())
                })))
            },
        ))),
    );
    prototype.insert(
        "toString".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Object.prototype.toString",
            None,
            |args| Ok(JsValue::String(object_to_string_tag(args.first()))),
        ))),
    );
    let prototype =
        js_prototypes::register("Object", JsValue::Object(Rc::new(RefCell::new(prototype))));
    JsValue::Native(Rc::new(
        NativeFunction::new("Object", None, |_| {
            Ok(JsValue::Object(Rc::new(RefCell::new(HashMap::new()))))
        })
        .with_property(
            "defineProperty",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.defineProperty",
                Some(3),
                object_define_property,
            ))),
        )
        .with_property(
            "getOwnPropertyDescriptor",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.getOwnPropertyDescriptor",
                Some(2),
                object_get_own_property_descriptor,
            ))),
        )
        .with_property(
            "getOwnPropertyDescriptors",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.getOwnPropertyDescriptors",
                Some(1),
                object_get_own_property_descriptors,
            ))),
        )
        .with_property(
            "hasOwn",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.hasOwn",
                Some(2),
                object_static_has_own,
            ))),
        )
        .with_property(
            "hasOwnProperty",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.hasOwnProperty",
                Some(2),
                object_static_has_own,
            ))),
        )
        .with_property(
            "defineProperties",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.defineProperties",
                Some(2),
                object_define_properties,
            ))),
        )
        .with_property(
            "create",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.create",
                None,
                object_create,
            ))),
        )
        .with_property(
            "setPrototypeOf",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.setPrototypeOf",
                Some(2),
                object_set_prototype_of,
            ))),
        )
        .with_property(
            "keys",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.keys",
                Some(1),
                |args| {
                    Ok(JsValue::Array(Rc::new(RefCell::new(
                        own_keys(&args[0])
                            .into_iter()
                            .map(JsValue::String)
                            .collect(),
                    ))))
                },
            ))),
        )
        .with_property(
            "values",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.values",
                Some(1),
                object_values,
            ))),
        )
        .with_property(
            "entries",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.entries",
                Some(1),
                object_entries,
            ))),
        )
        .with_property(
            "fromEntries",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.fromEntries",
                Some(1),
                object_from_entries,
            ))),
        )
        .with_property(
            "getOwnPropertyNames",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.getOwnPropertyNames",
                Some(1),
                object_own_names,
            ))),
        )
        .with_property(
            "getPrototypeOf",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.getPrototypeOf",
                Some(1),
                object_get_prototype_of,
            ))),
        )
        .with_property(
            "assign",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.assign",
                None,
                object_assign,
            ))),
        )
        .with_property(
            "freeze",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.freeze",
                Some(1),
                |args| Ok(args[0].clone()),
            ))),
        )
        .with_property(
            "seal",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.seal",
                Some(1),
                |args| Ok(args[0].clone()),
            ))),
        )
        .with_property(
            "preventExtensions",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.preventExtensions",
                Some(1),
                |args| Ok(args[0].clone()),
            ))),
        )
        .with_property(
            "isFrozen",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.isFrozen",
                Some(1),
                object_is_frozen,
            ))),
        )
        .with_property(
            "isSealed",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.isSealed",
                Some(1),
                object_is_frozen,
            ))),
        )
        .with_property(
            "isExtensible",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Object.isExtensible",
                Some(1),
                |args| Ok(JsValue::Bool(is_object_like(&args[0]))),
            ))),
        )
        .with_property("prototype", prototype),
    ))
}

fn object_define_property(args: &[JsValue]) -> Result<JsValue, String> {
    let name = args[1].display();
    let descriptor = &args[2];
    let getter = get_property(descriptor, "get")?;
    let setter = get_property(descriptor, "set")?;
    if !matches!(getter, JsValue::Undefined) || !matches!(setter, JsValue::Undefined) {
        set_accessor_property(&args[0], &name, getter, setter);
    } else if has_own_property(descriptor, "value") {
        set_property(&args[0], &name, get_property(descriptor, "value")?)?;
    }
    Ok(args[0].clone())
}

fn object_define_properties(args: &[JsValue]) -> Result<JsValue, String> {
    for key in own_keys(&args[1]) {
        let descriptor = get_property(&args[1], &key)?;
        object_define_property(&[args[0].clone(), JsValue::String(key), descriptor])?;
    }
    Ok(args[0].clone())
}

fn object_static_has_own(args: &[JsValue]) -> Result<JsValue, String> {
    let offset = usize::from(matches!(
        args.first(),
        Some(JsValue::Native(native)) if native.name == "Object"
    ));
    Ok(JsValue::Bool(has_own_property(
        &args[offset],
        &args[offset + 1].display(),
    )))
}

fn object_own_names(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Array(Rc::new(RefCell::new(
        own_keys(&args[0])
            .into_iter()
            .map(JsValue::String)
            .collect(),
    ))))
}

fn object_values(args: &[JsValue]) -> Result<JsValue, String> {
    let mut out = Vec::new();
    for key in own_keys(&args[0]) {
        out.push(get_property(&args[0], &key)?);
    }
    Ok(JsValue::Array(Rc::new(RefCell::new(out))))
}

fn object_entries(args: &[JsValue]) -> Result<JsValue, String> {
    let mut out = Vec::new();
    for key in own_keys(&args[0]) {
        out.push(JsValue::Array(Rc::new(RefCell::new(vec![
            JsValue::String(key.clone()),
            get_property(&args[0], &key)?,
        ]))));
    }
    Ok(JsValue::Array(Rc::new(RefCell::new(out))))
}

fn object_from_entries(args: &[JsValue]) -> Result<JsValue, String> {
    let mut out = HashMap::new();
    for entry in for_of_values(args[0].clone()) {
        let key = get_property(&entry, "0")?.display();
        let value = get_property(&entry, "1")?;
        out.insert(key, value);
    }
    Ok(JsValue::Object(Rc::new(RefCell::new(out))))
}

fn object_is_frozen(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(JsValue::Bool(!is_object_like(&args[0])))
}

fn is_object_like(value: &JsValue) -> bool {
    matches!(
        value,
        JsValue::Array(_)
            | JsValue::Object(_)
            | JsValue::Function(_)
            | JsValue::BoundFunction(_)
            | JsValue::Class(_)
            | JsValue::Native(_)
    )
}

fn object_get_prototype_of(args: &[JsValue]) -> Result<JsValue, String> {
    Ok(explicit_proto(&args[0])
        .or_else(|| builtin_proto(&args[0]))
        .unwrap_or(JsValue::Null))
}

fn lookup_proto(value: &JsValue) -> Option<JsValue> {
    explicit_proto(value).or_else(|| builtin_proto(value))
}

fn explicit_proto(value: &JsValue) -> Option<JsValue> {
    match value {
        JsValue::Object(obj) => obj.borrow().get("__proto__").cloned(),
        JsValue::Array(items) => array_extra_property(items, "__proto__"),
        JsValue::Function(fun) => fun.properties.borrow().get("__proto__").cloned(),
        JsValue::Native(native) => native.property("__proto__"),
        JsValue::Class(class) => class.properties.borrow().get("__proto__").cloned(),
        _ => None,
    }
    .filter(|proto| !matches!(proto, JsValue::Undefined))
}

fn builtin_proto(value: &JsValue) -> Option<JsValue> {
    match value {
        JsValue::Object(_) => js_prototypes::get("Object"),
        JsValue::Array(_) => js_prototypes::get("Array"),
        JsValue::String(_) => js_prototypes::get("String"),
        JsValue::Symbol(_) => js_prototypes::get("Symbol"),
        JsValue::Number(_) => js_prototypes::get("Number"),
        JsValue::Bool(_) => js_prototypes::get("Boolean"),
        JsValue::Function(_)
        | JsValue::BoundFunction(_)
        | JsValue::Native(_)
        | JsValue::Class(_) => js_prototypes::get("Function"),
        _ => None,
    }
}

fn object_set_prototype_of(args: &[JsValue]) -> Result<JsValue, String> {
    let target = args.first().cloned().unwrap_or(JsValue::Undefined);
    let prototype = args.get(1).cloned().unwrap_or(JsValue::Null);
    if !matches!(prototype, JsValue::Null) && !is_object_like(&prototype) {
        return Err("TypeError: Object prototype may only be an object or null".into());
    }
    set_property(&target, "__proto__", prototype)?;
    Ok(target)
}

fn object_get_own_property_descriptor(args: &[JsValue]) -> Result<JsValue, String> {
    let name = args[1].display();
    if !has_own_property(&args[0], &name) {
        return Ok(JsValue::Undefined);
    }
    let mut out = HashMap::new();
    if let Some(getter) = accessor_property(&args[0], "get", &name) {
        out.insert("get".into(), getter);
    } else {
        out.insert("value".into(), get_property(&args[0], &name)?);
        out.insert("writable".into(), JsValue::Bool(true));
    }
    if let Some(setter) = accessor_property(&args[0], "set", &name) {
        out.insert("set".into(), setter);
    }
    out.insert("enumerable".into(), JsValue::Bool(true));
    out.insert("configurable".into(), JsValue::Bool(true));
    Ok(JsValue::Object(Rc::new(RefCell::new(out))))
}

fn accessor_property(value: &JsValue, kind: &str, name: &str) -> Option<JsValue> {
    match value {
        JsValue::Object(obj) => obj.borrow().get(&accessor_key(kind, name)).cloned(),
        JsValue::Array(items) => array_extra_property(items, &accessor_key(kind, name)),
        JsValue::Function(fun) => fun
            .properties
            .borrow()
            .get(&accessor_key(kind, name))
            .cloned(),
        JsValue::Native(native) => native
            .properties
            .borrow()
            .get(&accessor_key(kind, name))
            .cloned(),
        JsValue::Class(class) => class
            .properties
            .borrow()
            .get(&accessor_key(kind, name))
            .cloned(),
        _ => None,
    }
}

fn accessor_key(kind: &str, name: &str) -> String {
    format!("__{kind}:{name}")
}

fn set_accessor_property(target: &JsValue, name: &str, getter: JsValue, setter: JsValue) {
    if !matches!(getter, JsValue::Undefined) {
        set_raw_property(target, accessor_key("get", name), getter);
    }
    if !matches!(setter, JsValue::Undefined) {
        set_raw_property(target, accessor_key("set", name), setter);
    }
}

fn set_raw_property(target: &JsValue, name: String, value: JsValue) {
    match target {
        JsValue::Object(obj) => {
            obj.borrow_mut().insert(name, value);
        }
        JsValue::Array(items) => set_array_extra_property(items, &name, value),
        JsValue::Function(fun) => {
            fun.properties.borrow_mut().insert(name, value);
        }
        JsValue::Native(native) => {
            native.properties.borrow_mut().insert(name, value);
        }
        JsValue::Class(class) => {
            class.properties.borrow_mut().insert(name, value);
        }
        _ => {}
    }
}

fn object_get_own_property_descriptors(args: &[JsValue]) -> Result<JsValue, String> {
    let mut out = HashMap::new();
    for key in own_keys(&args[0]) {
        let descriptor =
            object_get_own_property_descriptor(&[args[0].clone(), JsValue::String(key.clone())])?;
        out.insert(key, descriptor);
    }
    Ok(JsValue::Object(Rc::new(RefCell::new(out))))
}

fn object_to_string_tag(value: Option<&JsValue>) -> String {
    let tag = match value {
        Some(JsValue::Undefined) | None => "Undefined".into(),
        Some(JsValue::Null) => "Null".into(),
        Some(JsValue::Bool(_)) => "Boolean".into(),
        Some(JsValue::Number(_)) => "Number".into(),
        Some(JsValue::String(_)) => "String".into(),
        Some(JsValue::Symbol(_)) => "Symbol".into(),
        Some(JsValue::Array(_)) => "Array".into(),
        Some(JsValue::Function(_))
        | Some(JsValue::BoundFunction(_))
        | Some(JsValue::Class(_))
        | Some(JsValue::Native(_)) => "Function".into(),
        Some(JsValue::Object(object)) => object
            .borrow()
            .get("Symbol.toStringTag")
            .and_then(|value| match value {
                JsValue::String(tag) => Some(tag.clone()),
                _ => None,
            })
            .unwrap_or_else(|| "Object".into()),
    };
    format!("[object {tag}]")
}

fn object_create(args: &[JsValue]) -> Result<JsValue, String> {
    let out = Rc::new(RefCell::new(HashMap::new()));
    if let Some(proto) = args.first() {
        out.borrow_mut().insert("__proto__".into(), proto.clone());
    }
    if let Some(descriptors) = args.get(1) {
        for key in own_keys(descriptors) {
            let descriptor = get_property(descriptors, &key)?;
            object_define_property(&[
                JsValue::Object(out.clone()),
                JsValue::String(key),
                descriptor,
            ])?;
        }
    }
    Ok(JsValue::Object(out))
}

fn object_assign(args: &[JsValue]) -> Result<JsValue, String> {
    let target = args.first().cloned().unwrap_or(JsValue::Undefined);
    for source in args.iter().skip(1) {
        for key in own_keys(source) {
            set_property(&target, &key, get_property(source, &key)?)?;
        }
    }
    Ok(target)
}

fn array_constructor() -> JsValue {
    let mut prototype = HashMap::new();
    prototype.insert(
        "slice".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Array.prototype.slice",
            None,
            array_prototype_slice,
        ))),
    );
    js_array_prototype::install(&mut prototype);
    let prototype =
        js_prototypes::register("Array", JsValue::Object(Rc::new(RefCell::new(prototype))));
    JsValue::Native(Rc::new(
        NativeFunction::new("Array", None, |args| {
            if let [JsValue::Number(length)] = args {
                return Ok(JsValue::Array(Rc::new(RefCell::new(vec![
                    JsValue::Undefined;
                    *length as usize
                ]))));
            }
            Ok(JsValue::Array(Rc::new(RefCell::new(args.to_vec()))))
        })
        .with_property(
            "isArray",
            JsValue::Native(Rc::new(NativeFunction::new(
                "Array.isArray",
                Some(1),
                |args| Ok(JsValue::Bool(matches!(args[0], JsValue::Array(_)))),
            ))),
        )
        .with_property(
            "from",
            JsValue::Native(Rc::new(NativeFunction::new("Array.from", None, array_from))),
        )
        .with_property("prototype", prototype),
    ))
}

fn array_from(args: &[JsValue]) -> Result<JsValue, String> {
    let source = args.first().cloned().unwrap_or(JsValue::Undefined);
    let mapper = args.get(1).cloned().unwrap_or(JsValue::Undefined);
    let values = for_of_values(source);
    let mut out = Vec::with_capacity(values.len());
    for (index, value) in values.into_iter().enumerate() {
        if is_callable(&mapper) {
            out.push(call_value(
                mapper.clone(),
                &[value, JsValue::Number(index as f64)],
            )?);
        } else {
            out.push(value);
        }
    }
    Ok(JsValue::Array(Rc::new(RefCell::new(out))))
}

fn array_prototype_slice(args: &[JsValue]) -> Result<JsValue, String> {
    let receiver = args.first().cloned().unwrap_or(JsValue::Undefined);
    array_slice_values(
        js_array_like::values(&receiver),
        args.get(1..).unwrap_or(&[]),
    )
}

fn collection_constructor(name: &'static str) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(name, None, move |args| {
        Ok(collection_object(name, args.first()))
    })))
}

fn collection_object(kind: &'static str, initial: Option<&JsValue>) -> JsValue {
    let values = Rc::new(RefCell::new(Vec::<(JsValue, JsValue)>::new()));
    seed_collection(kind, &values, initial);
    let obj = Rc::new(RefCell::new(HashMap::new()));
    let add_values = values.clone();
    let add_object = obj.clone();
    obj.borrow_mut().insert(
        "add".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Set.add",
            Some(1),
            move |args| {
                add_values
                    .borrow_mut()
                    .push((args[0].clone(), JsValue::Bool(true)));
                Ok(JsValue::Object(add_object.clone()))
            },
        ))),
    );
    let has_values = values.clone();
    obj.borrow_mut().insert(
        "has".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Set.has",
            Some(1),
            move |args| {
                Ok(JsValue::Bool(
                    has_values.borrow().iter().any(|(key, _)| key == &args[0]),
                ))
            },
        ))),
    );
    let set_values = values.clone();
    let set_object = obj.clone();
    obj.borrow_mut().insert(
        "set".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Map.set",
            Some(2),
            move |args| {
                set_values
                    .borrow_mut()
                    .push((args[0].clone(), args[1].clone()));
                Ok(JsValue::Object(set_object.clone()))
            },
        ))),
    );
    let get_values = values.clone();
    obj.borrow_mut().insert(
        "get".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Map.get",
            Some(1),
            move |args| {
                Ok(get_values
                    .borrow()
                    .iter()
                    .rev()
                    .find(|(key, _)| key == &args[0])
                    .map(|(_, value)| value.clone())
                    .unwrap_or(JsValue::Undefined))
            },
        ))),
    );
    install_collection_methods(kind, &obj, values);
    JsValue::Object(obj)
}

fn seed_collection(
    kind: &'static str,
    values: &Rc<RefCell<Vec<(JsValue, JsValue)>>>,
    initial: Option<&JsValue>,
) {
    let Some(JsValue::Array(items)) = initial else {
        return;
    };
    for item in items.borrow().iter() {
        if kind.contains("Map") {
            if let JsValue::Array(pair) = item {
                let pair = pair.borrow();
                values.borrow_mut().push((
                    pair.first().cloned().unwrap_or(JsValue::Undefined),
                    pair.get(1).cloned().unwrap_or(JsValue::Undefined),
                ));
            }
        } else {
            values
                .borrow_mut()
                .push((item.clone(), JsValue::Bool(true)));
        }
    }
}

fn install_collection_methods(
    kind: &'static str,
    obj: &Rc<RefCell<HashMap<String, JsValue>>>,
    values: Rc<RefCell<Vec<(JsValue, JsValue)>>>,
) {
    let each_values = values.clone();
    let each_object = obj.clone();
    obj.borrow_mut().insert(
        "forEach".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Collection.forEach",
            None,
            move |args| collection_for_each(kind, each_values.clone(), each_object.clone(), args),
        ))),
    );
    let size_values = values.clone();
    obj.borrow_mut().insert(
        "__get:size".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Collection.size",
            Some(0),
            move |_| Ok(JsValue::Number(size_values.borrow().len() as f64)),
        ))),
    );
    let delete_values = values.clone();
    obj.borrow_mut().insert(
        "delete".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Collection.delete",
            Some(1),
            move |args| {
                Ok(JsValue::Bool(delete_collection_value(
                    &delete_values,
                    &args[0],
                )))
            },
        ))),
    );
    obj.borrow_mut().insert(
        "clear".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Collection.clear",
            Some(0),
            move |_| {
                values.borrow_mut().clear();
                Ok(JsValue::Undefined)
            },
        ))),
    );
}

fn collection_for_each(
    kind: &'static str,
    values: Rc<RefCell<Vec<(JsValue, JsValue)>>>,
    object: Rc<RefCell<HashMap<String, JsValue>>>,
    args: &[JsValue],
) -> Result<JsValue, String> {
    let callback = args
        .first()
        .cloned()
        .ok_or_else(|| "Collection.forEach: expected callback".to_string())?;
    let object = JsValue::Object(object);
    for (key, value) in values.borrow().clone() {
        let first = if kind.contains("Map") {
            value
        } else {
            key.clone()
        };
        call_value(callback.clone(), &[first, key, object.clone()])?;
    }
    Ok(JsValue::Undefined)
}

fn delete_collection_value(values: &Rc<RefCell<Vec<(JsValue, JsValue)>>>, key: &JsValue) -> bool {
    let mut values = values.borrow_mut();
    let before = values.len();
    values.retain(|(item, _)| item != key);
    values.len() != before
}

fn promise_resolve(args: &[JsValue]) -> Result<JsValue, String> {
    let value = args.first().cloned().unwrap_or(JsValue::Undefined);
    let mut obj = HashMap::new();
    obj.insert(
        "__promise_state".into(),
        JsValue::String("fulfilled".into()),
    );
    obj.insert("value".into(), value.clone());
    install_then_catch(
        &mut obj,
        Rc::new(RefCell::new(PromiseState::Fulfilled(value))),
    );
    Ok(JsValue::Object(Rc::new(RefCell::new(obj))))
}

fn promise_reject(args: &[JsValue]) -> Result<JsValue, String> {
    let reason = args.first().cloned().unwrap_or(JsValue::Undefined);
    let mut obj = HashMap::new();
    obj.insert("__promise_state".into(), JsValue::String("rejected".into()));
    obj.insert("reason".into(), reason.clone());
    install_then_catch(
        &mut obj,
        Rc::new(RefCell::new(PromiseState::Rejected(reason))),
    );
    Ok(JsValue::Object(Rc::new(RefCell::new(obj))))
}

/// Internal promise state.
#[derive(Clone)]
enum PromiseState {
    Pending,
    Fulfilled(JsValue),
    Rejected(JsValue),
}

/// Create a new promise object from an executor function.
fn make_promise(executor: JsValue) -> Result<JsValue, String> {
    let state = Rc::new(RefCell::new(PromiseState::Pending));
    let obj_rc: Rc<RefCell<HashMap<String, JsValue>>> = Rc::new(RefCell::new(HashMap::new()));

    // resolve callback
    let s = state.clone();
    let o = obj_rc.clone();
    let resolve_fn = JsValue::Native(Rc::new(NativeFunction::new(
        "resolve",
        Some(1),
        move |args| {
            let val = args.first().cloned().unwrap_or(JsValue::Undefined);
            *s.borrow_mut() = PromiseState::Fulfilled(val.clone());
            o.borrow_mut().insert(
                "__promise_state".into(),
                JsValue::String("fulfilled".into()),
            );
            o.borrow_mut().insert("value".into(), val);
            Ok(JsValue::Undefined)
        },
    )));

    // reject callback
    let s2 = state.clone();
    let o2 = obj_rc.clone();
    let reject_fn = JsValue::Native(Rc::new(NativeFunction::new(
        "reject",
        Some(1),
        move |args| {
            let reason = args.first().cloned().unwrap_or(JsValue::Undefined);
            *s2.borrow_mut() = PromiseState::Rejected(reason.clone());
            o2.borrow_mut()
                .insert("__promise_state".into(), JsValue::String("rejected".into()));
            o2.borrow_mut().insert("reason".into(), reason);
            Ok(JsValue::Undefined)
        },
    )));

    // Install .then / .catch on the shared object
    {
        let mut obj = obj_rc.borrow_mut();
        obj.insert("__promise_state".into(), JsValue::String("pending".into()));
        install_then_catch(&mut obj, state.clone());
    }

    // Execute the executor synchronously
    let _ = call_value(executor, &[resolve_fn, reject_fn]);

    // Return a snapshot of the object (after executor has run, state is known)
    let snapshot: HashMap<String, JsValue> = obj_rc.borrow().clone();
    Ok(JsValue::Object(Rc::new(RefCell::new(snapshot))))
}

/// Install `.then()` and `.catch()` methods on a promise object map.
fn install_then_catch(obj: &mut HashMap<String, JsValue>, state: Rc<RefCell<PromiseState>>) {
    let st = state.clone();
    obj.insert(
        "then".into(),
        JsValue::Native(Rc::new(NativeFunction::new("then", None, move |args| {
            let on_ok = args.first().cloned().unwrap_or(JsValue::Undefined);
            let on_err = args.get(1).cloned().unwrap_or(JsValue::Undefined);
            let current = st.borrow().clone();
            let mut next = HashMap::new();

            match &current {
                PromiseState::Fulfilled(val) => {
                    if on_ok.truthy() {
                        match call_value(on_ok, std::slice::from_ref(val)) {
                            Ok(result) => {
                                // If result is itself a promise-like object, propagate
                                if let JsValue::Object(ref robj) = result {
                                    if robj.borrow().contains_key("__promise_state") {
                                        return Ok(JsValue::Object(robj.clone()));
                                    }
                                }
                                next.insert(
                                    "__promise_state".into(),
                                    JsValue::String("fulfilled".into()),
                                );
                                next.insert("value".into(), result);
                            }
                            Err(e) => {
                                next.insert(
                                    "__promise_state".into(),
                                    JsValue::String("rejected".into()),
                                );
                                next.insert("reason".into(), JsValue::String(e));
                            }
                        }
                    } else {
                        next.insert(
                            "__promise_state".into(),
                            JsValue::String("fulfilled".into()),
                        );
                        next.insert("value".into(), val.clone());
                    }
                }
                PromiseState::Rejected(reason) => {
                    if on_err.truthy() {
                        match call_value(on_err, std::slice::from_ref(reason)) {
                            Ok(result) => {
                                next.insert(
                                    "__promise_state".into(),
                                    JsValue::String("fulfilled".into()),
                                );
                                next.insert("value".into(), result);
                            }
                            Err(e) => {
                                next.insert(
                                    "__promise_state".into(),
                                    JsValue::String("rejected".into()),
                                );
                                next.insert("reason".into(), JsValue::String(e));
                            }
                        }
                    } else {
                        next.insert("__promise_state".into(), JsValue::String("rejected".into()));
                        next.insert("reason".into(), reason.clone());
                    }
                }
                PromiseState::Pending => {
                    next.insert("__promise_state".into(), JsValue::String("pending".into()));
                }
            }

            let next_state = Rc::new(RefCell::new(
                match next.get("__promise_state").and_then(|v| {
                    if let JsValue::String(s) = v {
                        Some(s.as_str())
                    } else {
                        None
                    }
                }) {
                    Some("fulfilled") => PromiseState::Fulfilled(
                        next.get("value").cloned().unwrap_or(JsValue::Undefined),
                    ),
                    Some("rejected") => PromiseState::Rejected(
                        next.get("reason").cloned().unwrap_or(JsValue::Undefined),
                    ),
                    _ => PromiseState::Pending,
                },
            ));
            install_then_catch(&mut next, next_state);
            Ok(JsValue::Object(Rc::new(RefCell::new(next))))
        }))),
    );

    let st2 = state.clone();
    obj.insert(
        "catch".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "catch",
            Some(1),
            move |args| {
                let handler = args.first().cloned().unwrap_or(JsValue::Undefined);
                let current = st2.borrow().clone();
                let mut next = HashMap::new();

                match &current {
                    PromiseState::Rejected(reason) => {
                        if handler.truthy() {
                            match call_value(handler, std::slice::from_ref(reason)) {
                                Ok(result) => {
                                    next.insert(
                                        "__promise_state".into(),
                                        JsValue::String("fulfilled".into()),
                                    );
                                    next.insert("value".into(), result);
                                }
                                Err(e) => {
                                    next.insert(
                                        "__promise_state".into(),
                                        JsValue::String("rejected".into()),
                                    );
                                    next.insert("reason".into(), JsValue::String(e));
                                }
                            }
                        } else {
                            next.insert(
                                "__promise_state".into(),
                                JsValue::String("rejected".into()),
                            );
                        }
                    }
                    PromiseState::Fulfilled(val) => {
                        next.insert(
                            "__promise_state".into(),
                            JsValue::String("fulfilled".into()),
                        );
                        next.insert("value".into(), val.clone());
                    }
                    PromiseState::Pending => {
                        next.insert("__promise_state".into(), JsValue::String("pending".into()));
                    }
                }

                let next_state = Rc::new(RefCell::new(
                    match next.get("__promise_state").and_then(|v| {
                        if let JsValue::String(s) = v {
                            Some(s.as_str())
                        } else {
                            None
                        }
                    }) {
                        Some("fulfilled") => PromiseState::Fulfilled(
                            next.get("value").cloned().unwrap_or(JsValue::Undefined),
                        ),
                        Some("rejected") => PromiseState::Rejected(
                            next.get("reason").cloned().unwrap_or(JsValue::Undefined),
                        ),
                        _ => PromiseState::Pending,
                    },
                ));
                install_then_catch(&mut next, next_state);
                Ok(JsValue::Object(Rc::new(RefCell::new(next))))
            },
        ))),
    );
}

fn array_global_functions() -> Vec<(&'static str, JsValue)> {
    vec![
        ("push", native_array_global("push", array_push)),
        (
            "pop",
            native_array_global("pop", |array, _| array_pop(array)),
        ),
        ("slice", native_array_global("slice", array_slice)),
        ("join", native_array_global("join", array_join)),
        ("forEach", native_array_global("forEach", array_for_each)),
        ("map", native_array_global("map", array_map)),
    ]
}

fn native_array_global(
    name: &'static str,
    func: impl Fn(Rc<RefCell<Vec<JsValue>>>, &[JsValue]) -> Result<JsValue, String> + 'static,
) -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new(name, None, move |args| {
        let Some(JsValue::Array(array)) = args.first() else {
            return Err(format!("{}: expected array as first arg", name));
        };
        func(array.clone(), &args[1..])
    })))
}

type EnvRef = Rc<RefCell<Env>>;

#[derive(Clone)]
struct Env {
    values: HashMap<String, JsValue>,
    parent: Option<EnvRef>,
}

impl Env {
    fn new(parent: Option<EnvRef>) -> EnvRef {
        Rc::new(RefCell::new(Self {
            values: HashMap::new(),
            parent,
        }))
    }

    fn define(&mut self, name: &str, value: JsValue) {
        self.values.insert(name.into(), value);
    }

    fn get(&self, name: &str) -> Option<JsValue> {
        self.values
            .get(name)
            .cloned()
            .or_else(|| self.parent.as_ref().and_then(|p| p.borrow().get(name)))
    }

    fn assign(env: &EnvRef, name: &str, value: JsValue) -> Result<(), String> {
        if env.borrow().values.contains_key(name) {
            env.borrow_mut().values.insert(name.into(), value);
            return Ok(());
        }
        if let Some(parent) = env.borrow().parent.clone() {
            return Env::assign(&parent, name, value);
        }
        Err(format!("ReferenceError: {} is not defined", name))
    }
}

#[derive(Debug, Clone, PartialEq)]
enum TokenKind {
    Number(f64),
    String(String),
    Ident(String),
    Let,
    Const,
    Var,
    Class,
    Extends,
    Function,
    Return,
    Throw,
    If,
    Else,
    Switch,
    Case,
    Default,
    While,
    Do,
    Delete,
    For,
    Try,
    Catch,
    Finally,
    In,
    Instanceof,
    Of,
    New,
    Break,
    Continue,
    True,
    False,
    Null,
    This,
    Typeof,
    Void,
    Plus,
    PlusEq,
    PlusPlus,
    Minus,
    MinusEq,
    MinusMinus,
    Star,
    StarStar,
    StarEq,
    Slash,
    SlashEq,
    Percent,
    PercentEq,
    Bang,
    Eq,
    FatArrow,
    EqEq,
    BangEq,
    StrictEq,
    StrictBangEq,
    Nullish,
    OptionalChain,
    Lt,
    Lte,
    Gt,
    Gte,
    AndAnd,
    OrOr,
    BitAnd,
    BitAndEq,
    BitOr,
    BitOrEq,
    Caret,
    CaretEq,
    Tilde,
    Shl,
    ShlEq,
    Shr,
    ShrEq,
    UShr,
    UShrEq,
    Dot,
    Spread,
    Comma,
    Semi,
    Colon,
    Question,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,
    Eof,
}

#[derive(Debug, Clone)]
struct Token {
    kind: TokenKind,
    line: usize,
    col: usize,
}

struct Lexer {
    chars: Vec<char>,
    pos: usize,
    line: usize,
    col: usize,
}

impl Lexer {
    fn new(src: &str) -> Self {
        Self {
            chars: src.chars().collect(),
            pos: 0,
            line: 1,
            col: 1,
        }
    }

    fn tokenize(mut self) -> Result<Vec<Token>, String> {
        let mut out = Vec::new();
        loop {
            let tok = self.next_token()?;
            let eof = tok.kind == TokenKind::Eof;
            out.push(tok);
            if eof {
                break;
            }
        }
        Ok(out)
    }

    fn next_token(&mut self) -> Result<Token, String> {
        self.skip_ws_and_comments();
        let line = self.line;
        let col = self.col;
        let Some(c) = self.advance() else {
            return Ok(Token {
                kind: TokenKind::Eof,
                line,
                col,
            });
        };
        let kind = match c {
            '0'..='9' => self.number(c)?,
            '"' | '\'' | '`' => self.string(c, line, col)?,
            c if c == '_' || c == '$' || c.is_ascii_alphabetic() || !c.is_ascii() => self.ident(c),
            '+' => {
                if self.match_char('+') {
                    TokenKind::PlusPlus
                } else if self.match_char('=') {
                    TokenKind::PlusEq
                } else {
                    TokenKind::Plus
                }
            }
            '-' => {
                if self.match_char('-') {
                    TokenKind::MinusMinus
                } else if self.match_char('=') {
                    TokenKind::MinusEq
                } else {
                    TokenKind::Minus
                }
            }
            '*' => {
                if self.match_char('*') {
                    TokenKind::StarStar
                } else if self.match_char('=') {
                    TokenKind::StarEq
                } else {
                    TokenKind::Star
                }
            }
            '/' => {
                if self.match_char('=') {
                    TokenKind::SlashEq
                } else {
                    TokenKind::Slash
                }
            }
            '%' => {
                if self.match_char('=') {
                    TokenKind::PercentEq
                } else {
                    TokenKind::Percent
                }
            }
            '!' => {
                if self.match_char('=') {
                    if self.match_char('=') {
                        TokenKind::StrictBangEq
                    } else {
                        TokenKind::BangEq
                    }
                } else {
                    TokenKind::Bang
                }
            }
            '=' => {
                if self.match_char('>') {
                    TokenKind::FatArrow
                } else if self.match_char('=') {
                    if self.match_char('=') {
                        TokenKind::StrictEq
                    } else {
                        TokenKind::EqEq
                    }
                } else {
                    TokenKind::Eq
                }
            }
            '<' => {
                if self.match_char('<') {
                    if self.match_char('=') {
                        TokenKind::ShlEq
                    } else {
                        TokenKind::Shl
                    }
                } else if self.match_char('=') {
                    TokenKind::Lte
                } else {
                    TokenKind::Lt
                }
            }
            '>' => {
                if self.match_char('>') {
                    if self.match_char('>') {
                        if self.match_char('=') {
                            TokenKind::UShrEq
                        } else {
                            TokenKind::UShr
                        }
                    } else if self.match_char('=') {
                        TokenKind::ShrEq
                    } else {
                        TokenKind::Shr
                    }
                } else if self.match_char('=') {
                    TokenKind::Gte
                } else {
                    TokenKind::Gt
                }
            }
            '&' => {
                if self.match_char('&') {
                    TokenKind::AndAnd
                } else if self.match_char('=') {
                    TokenKind::BitAndEq
                } else {
                    TokenKind::BitAnd
                }
            }
            '|' => {
                if self.match_char('|') {
                    TokenKind::OrOr
                } else if self.match_char('=') {
                    TokenKind::BitOrEq
                } else {
                    TokenKind::BitOr
                }
            }
            '^' => {
                if self.match_char('=') {
                    TokenKind::CaretEq
                } else {
                    TokenKind::Caret
                }
            }
            '~' => TokenKind::Tilde,
            '.' => {
                if self.peek() == Some('.') && self.peek_next() == Some('.') {
                    self.advance();
                    self.advance();
                    TokenKind::Spread
                } else if matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                    self.number('.')?
                } else {
                    TokenKind::Dot
                }
            }
            ',' => TokenKind::Comma,
            ';' => TokenKind::Semi,
            ':' => TokenKind::Colon,
            '?' => {
                if self.peek() == Some('.')
                    && matches!(self.peek_next(), Some(c) if c.is_ascii_digit())
                {
                    TokenKind::Question
                } else if self.match_char('.') {
                    TokenKind::OptionalChain
                } else if self.match_char('?') {
                    TokenKind::Nullish
                } else {
                    TokenKind::Question
                }
            }
            '(' => TokenKind::LParen,
            ')' => TokenKind::RParen,
            '{' => TokenKind::LBrace,
            '}' => TokenKind::RBrace,
            '[' => TokenKind::LBracket,
            ']' => TokenKind::RBracket,
            other => {
                return Err(format!(
                    "Unexpected character '{}' at {}:{} near {}",
                    other,
                    line,
                    col,
                    self.snippet()
                ))
            }
        };
        Ok(Token { kind, line, col })
    }

    fn skip_ws_and_comments(&mut self) {
        loop {
            while matches!(self.peek(), Some(c) if c.is_whitespace()) {
                self.advance();
            }
            if self.peek() == Some('/') && self.peek_next() == Some('/') {
                while !matches!(self.peek(), None | Some('\n')) {
                    self.advance();
                }
                continue;
            }
            if self.peek() == Some('/') && self.peek_next() == Some('*') {
                self.advance();
                self.advance();
                while !(self.peek() == Some('*') && self.peek_next() == Some('/'))
                    && self.peek().is_some()
                {
                    self.advance();
                }
                if self.peek().is_some() {
                    self.advance();
                    self.advance();
                }
                continue;
            }
            break;
        }
    }

    fn number(&mut self, first: char) -> Result<TokenKind, String> {
        let mut s = String::new();
        if first == '.' {
            s.push('0');
        }
        s.push(first);
        while matches!(self.peek(), Some(c) if c.is_ascii_digit() || c == '.') {
            s.push(self.advance().unwrap());
        }
        if matches!(self.peek(), Some('e' | 'E')) {
            s.push(self.advance().unwrap());
            if matches!(self.peek(), Some('+' | '-')) {
                s.push(self.advance().unwrap());
            }
            while matches!(self.peek(), Some(c) if c.is_ascii_digit()) {
                s.push(self.advance().unwrap());
            }
        }
        Ok(TokenKind::Number(
            s.parse().map_err(|_| format!("Invalid number {}", s))?,
        ))
    }

    fn string(&mut self, quote: char, line: usize, col: usize) -> Result<TokenKind, String> {
        let mut s = String::new();
        loop {
            let Some(c) = self.advance() else {
                return Err(format!("Unterminated string starting at {}:{}", line, col));
            };
            if c == quote {
                break;
            }
            if c == '\\' {
                let Some(e) = self.advance() else {
                    return Err(format!(
                        "Unterminated string escape starting at {}:{}",
                        line, col
                    ));
                };
                s.push(match e {
                    'n' => '\n',
                    'r' => '\r',
                    't' => '\t',
                    '\\' => '\\',
                    '\'' => '\'',
                    '"' => '"',
                    other => other,
                });
            } else {
                s.push(c);
            }
        }
        Ok(TokenKind::String(s))
    }

    fn ident(&mut self, first: char) -> TokenKind {
        let mut s = String::new();
        s.push(first);
        while matches!(self.peek(), Some(c) if c.is_ascii_alphanumeric() || c == '_' || c == '$' || !c.is_ascii())
        {
            s.push(self.advance().unwrap());
        }
        match s.as_str() {
            "let" => TokenKind::Let,
            "const" => TokenKind::Const,
            "var" => TokenKind::Var,
            "class" => TokenKind::Class,
            "extends" => TokenKind::Extends,
            "function" => TokenKind::Function,
            "return" => TokenKind::Return,
            "throw" => TokenKind::Throw,
            "if" => TokenKind::If,
            "else" => TokenKind::Else,
            "switch" => TokenKind::Switch,
            "case" => TokenKind::Case,
            "default" => TokenKind::Default,
            "while" => TokenKind::While,
            "do" => TokenKind::Do,
            "delete" => TokenKind::Delete,
            "for" => TokenKind::For,
            "try" => TokenKind::Try,
            "catch" => TokenKind::Catch,
            "finally" => TokenKind::Finally,
            "in" => TokenKind::In,
            "instanceof" => TokenKind::Instanceof,
            "of" => TokenKind::Of,
            "new" => TokenKind::New,
            "break" => TokenKind::Break,
            "continue" => TokenKind::Continue,
            "true" => TokenKind::True,
            "false" => TokenKind::False,
            "null" => TokenKind::Null,
            "this" => TokenKind::This,
            "typeof" => TokenKind::Typeof,
            "void" => TokenKind::Void,
            _ => TokenKind::Ident(s),
        }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }
    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.pos + 1).copied()
    }

    fn snippet(&self) -> String {
        let mark = self.pos.saturating_sub(1);
        let start = mark.saturating_sub(40);
        let end = self.pos.saturating_add(80).min(self.chars.len());
        let mut out = String::new();
        for index in start..end {
            if index == mark {
                out.push_str("<<");
            }
            out.push(self.chars[index]);
            if index == mark {
                out.push_str(">>");
            }
        }
        out
    }
    fn match_char(&mut self, c: char) -> bool {
        if self.peek() == Some(c) {
            self.advance();
            true
        } else {
            false
        }
    }
    fn advance(&mut self) -> Option<char> {
        let c = self.peek()?;
        self.pos += 1;
        if c == '\n' {
            self.line += 1;
            self.col = 1;
        } else {
            self.col += 1;
        }
        Some(c)
    }
}

#[derive(Debug, Clone)]
enum Stmt {
    Expr(Expr),
    Empty,
    Many(Vec<Stmt>),
    Var(String, Option<Expr>),
    Vars(Vec<(String, Option<Expr>)>),
    Destructure(Vec<(String, String, Option<Expr>)>, Expr),
    ArrayDestructure(Vec<(String, String, Option<Expr>, bool)>, Expr),
    Function(String, Vec<String>, Vec<Stmt>),
    Return(Option<Expr>),
    Throw(Expr),
    Block(Vec<Stmt>),
    Label(String, Box<Stmt>),
    If(Expr, Box<Stmt>, Option<Box<Stmt>>),
    Switch(Expr, Vec<(Option<Expr>, Vec<Stmt>)>),
    While(Expr, Box<Stmt>),
    DoWhile(Box<Stmt>, Expr),
    For(Option<Box<Stmt>>, Option<Expr>, Option<Expr>, Box<Stmt>),
    ForIn(String, Expr, Box<Stmt>),
    ForOf(String, Expr, Box<Stmt>),
    Try(Vec<Stmt>, Option<(String, Vec<Stmt>)>, Option<Vec<Stmt>>),
    Break(Option<String>),
    Continue(Option<String>),
}
#[derive(Debug, Clone)]
enum Expr {
    Literal(JsValue),
    Var(String),
    This,
    Class(ClassExpr),
    Function(Option<String>, Vec<String>, Vec<Stmt>),
    Array(Vec<Expr>),
    Object(Vec<(String, Expr)>),
    Spread(Box<Expr>),
    Await(Box<Expr>),
    Unary(String, Box<Expr>),
    Typeof(Box<Expr>),
    Delete(Box<Expr>),
    Update(Box<Expr>, i32, bool),
    Binary(Box<Expr>, String, Box<Expr>),
    Conditional(Box<Expr>, Box<Expr>, Box<Expr>),
    Sequence(Vec<Expr>),
    Assign(Box<Expr>, Box<Expr>),
    AssignOp(Box<Expr>, String, Box<Expr>),
    Call(Box<Expr>, Vec<Expr>),
    New(Box<Expr>, Vec<Expr>),
    Get(Box<Expr>, String),
    OptionalGet(Box<Expr>, String),
    Index(Box<Expr>, Box<Expr>),
    OptionalIndex(Box<Expr>, Box<Expr>),
    OptionalCall(Box<Expr>, Vec<Expr>),
}

#[derive(Clone)]
struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

#[derive(Debug, Clone)]
struct ClassExpr {
    name: Option<String>,
    superclass: Option<Box<Expr>>,
    methods: Vec<ClassMethodExpr>,
}

#[derive(Debug, Clone)]
struct ClassMethodExpr {
    name: String,
    params: Vec<String>,
    body: Vec<Stmt>,
    is_static: bool,
}

type ArrayBinding = (String, String, Option<Expr>, bool);

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }
    fn parse_program(&mut self) -> Result<Vec<Stmt>, String> {
        let mut s = Vec::new();
        while !self.is_eof() {
            s.push(self.statement()?);
        }
        Ok(s)
    }

    fn statement(&mut self) -> Result<Stmt, String> {
        if self.matches(&[TokenKind::Semi]) {
            return Ok(Stmt::Empty);
        }
        if self.matches(&[TokenKind::Let, TokenKind::Const, TokenKind::Var]) {
            return self.var_decl();
        }
        if matches!(&self.peek().kind, TokenKind::Ident(name) if name == "async")
            && matches!(
                self.tokens.get(self.pos + 1).map(|token| &token.kind),
                Some(TokenKind::Function)
            )
        {
            self.advance();
        }
        if self.matches(&[TokenKind::Function]) {
            return self.function_decl();
        }
        if self.matches(&[TokenKind::Class]) {
            return self.class_decl();
        }
        if self.matches(&[TokenKind::Return]) {
            let expr = if self.check(&TokenKind::Semi) || self.check(&TokenKind::RBrace) {
                None
            } else {
                Some(self.comma_expression()?)
            };
            self.consume_optional_semi();
            return Ok(Stmt::Return(expr));
        }
        if self.matches(&[TokenKind::Throw]) {
            let expr = self.comma_expression()?;
            self.consume_optional_semi();
            return Ok(Stmt::Throw(expr));
        }
        if self.matches(&[TokenKind::If]) {
            return self.if_stmt();
        }
        if self.matches(&[TokenKind::Switch]) {
            return self.switch_stmt();
        }
        if self.matches(&[TokenKind::While]) {
            return self.while_stmt();
        }
        if self.matches(&[TokenKind::Do]) {
            return self.do_while_stmt();
        }
        if self.matches(&[TokenKind::For]) {
            return self.for_stmt();
        }
        if self.matches(&[TokenKind::Try]) {
            return self.try_stmt();
        }
        if self.matches(&[TokenKind::LBrace]) {
            return Ok(Stmt::Block(self.block()?));
        }
        if self.matches(&[TokenKind::Break]) {
            let label = self.consume_optional_label();
            self.consume_optional_semi();
            return Ok(Stmt::Break(label));
        }
        if self.matches(&[TokenKind::Continue]) {
            let label = self.consume_optional_label();
            self.consume_optional_semi();
            return Ok(Stmt::Continue(label));
        }
        if let Some(label) = self.label_name() {
            return Ok(Stmt::Label(label, Box::new(self.statement()?)));
        }
        let expr = self.comma_expression()?;
        self.consume_optional_semi();
        Ok(Stmt::Expr(expr))
    }

    fn var_decl(&mut self) -> Result<Stmt, String> {
        let mut stmts = Vec::new();
        loop {
            stmts.push(self.var_decl_item()?);
            if !self.matches(&[TokenKind::Comma]) {
                break;
            }
        }
        self.consume_optional_semi();
        Ok(stmt_group(stmts))
    }

    fn var_decl_item(&mut self) -> Result<Stmt, String> {
        if matches!(self.peek().kind, TokenKind::LBrace) {
            let bindings = self.object_binding_pattern()?;
            self.consume(&TokenKind::Eq, "Expected '=' after destructuring pattern")?;
            let init = self.expression()?;
            return Ok(Stmt::Destructure(bindings, init));
        }
        if matches!(self.peek().kind, TokenKind::LBracket) {
            let bindings = self.array_binding_pattern()?;
            self.consume(&TokenKind::Eq, "Expected '=' after array binding pattern")?;
            let init = self.expression()?;
            return Ok(Stmt::ArrayDestructure(bindings, init));
        }
        let name = self.consume_ident("Expected variable name")?;
        let init = if self.matches(&[TokenKind::Eq]) {
            Some(self.expression()?)
        } else {
            None
        };
        Ok(Stmt::Var(name, init))
    }

    fn object_binding_pattern(&mut self) -> Result<Vec<(String, String, Option<Expr>)>, String> {
        self.object_binding_pattern_prefixed(String::new())
    }

    fn object_binding_pattern_prefixed(
        &mut self,
        prefix: String,
    ) -> Result<Vec<(String, String, Option<Expr>)>, String> {
        self.consume(&TokenKind::LBrace, "Expected object binding pattern")?;
        let mut bindings = Vec::new();
        while !self.matches(&[TokenKind::RBrace]) {
            if self.matches(&[TokenKind::Spread]) {
                let target = self.consume_ident("Expected rest binding")?;
                bindings.push(("__rest".into(), target, None));
                self.consume(&TokenKind::RBrace, "Expected '}' after rest binding")?;
                break;
            }
            let source = self.consume_property("Expected destructured property")?;
            let source_path = binding_path(&prefix, &source);
            let has_colon = self.matches(&[TokenKind::Colon]);
            if has_colon && matches!(self.peek().kind, TokenKind::LBrace) {
                bindings.extend(self.object_binding_pattern_prefixed(source_path)?);
            } else if has_colon && matches!(self.peek().kind, TokenKind::LBracket) {
                bindings.extend(
                    self.array_binding_pattern_prefixed(source_path)?
                        .into_iter()
                        .filter(|(_, _, _, rest)| !*rest)
                        .map(|(path, name, default, _)| (path, name, default)),
                );
            } else {
                let target = if has_colon {
                    self.consume_ident("Expected destructured binding")?
                } else {
                    source.clone()
                };
                let default = if self.matches(&[TokenKind::Eq]) {
                    Some(self.expression()?)
                } else {
                    None
                };
                bindings.push((source_path, target, default));
            }
            if !self.matches(&[TokenKind::Comma]) {
                self.consume(&TokenKind::RBrace, "Expected '}' after binding pattern")?;
                break;
            }
        }
        Ok(bindings)
    }

    fn array_binding_pattern(&mut self) -> Result<Vec<ArrayBinding>, String> {
        self.array_binding_pattern_prefixed(String::new())
    }

    fn array_binding_pattern_prefixed(
        &mut self,
        prefix: String,
    ) -> Result<Vec<ArrayBinding>, String> {
        self.consume(&TokenKind::LBracket, "Expected array binding pattern")?;
        let mut bindings = Vec::new();
        let mut index = 0;
        while !self.matches(&[TokenKind::RBracket]) {
            if self.matches(&[TokenKind::Comma]) {
                index += 1;
                continue;
            }
            let rest = self.matches(&[TokenKind::Spread]);
            let path = binding_path(&prefix, &index.to_string());
            if matches!(self.peek().kind, TokenKind::LBrace) {
                bindings.extend(
                    self.object_binding_pattern_prefixed(path)?
                        .into_iter()
                        .map(|(path, name, default)| (path, name, default, false)),
                );
                if !self.matches(&[TokenKind::Comma]) {
                    self.consume(&TokenKind::RBracket, "Expected ']' after binding pattern")?;
                    break;
                }
                index += 1;
                continue;
            }
            let name = self.consume_ident("Expected array binding name")?;
            let default = if !rest && self.matches(&[TokenKind::Eq]) {
                Some(self.expression()?)
            } else {
                None
            };
            bindings.push((path, name, default, rest));
            if rest || !self.matches(&[TokenKind::Comma]) {
                self.consume(&TokenKind::RBracket, "Expected ']' after binding pattern")?;
                break;
            }
            index += 1;
        }
        Ok(bindings)
    }

    fn function_decl(&mut self) -> Result<Stmt, String> {
        self.matches(&[TokenKind::Star]);
        let name = self.consume_ident("Expected function name")?;
        self.consume(&TokenKind::LParen, "Expected '(' after function name")?;
        let (params, defaults) = self.params()?;
        self.consume(&TokenKind::LBrace, "Expected function body")?;
        Ok(Stmt::Function(
            name,
            params,
            with_param_defaults(defaults, self.block()?),
        ))
    }

    fn class_decl(&mut self) -> Result<Stmt, String> {
        let name = self.consume_ident("Expected class name")?;
        Ok(Stmt::Var(name.clone(), Some(self.class_expr(Some(name))?)))
    }

    fn class_expr(&mut self, name: Option<String>) -> Result<Expr, String> {
        let superclass = if self.matches(&[TokenKind::Extends]) {
            Some(Box::new(self.expression()?))
        } else {
            None
        };
        self.consume(&TokenKind::LBrace, "Expected class body")?;
        let mut methods = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_eof() {
            let is_static = self.static_method_marker();
            self.async_method_marker();
            self.matches(&[TokenKind::Star]);
            let accessor = self.accessor_marker();
            let method_name = self.method_name()?;
            self.consume(&TokenKind::LParen, "Expected method parameters")?;
            let (params, defaults) = self.params()?;
            self.consume(&TokenKind::LBrace, "Expected method body")?;
            methods.push(ClassMethodExpr {
                name: accessor_name(accessor, method_name),
                params,
                body: with_param_defaults(defaults, self.block()?),
                is_static,
            });
        }
        self.consume(&TokenKind::RBrace, "Expected class body end")?;
        Ok(Expr::Class(ClassExpr {
            name,
            superclass,
            methods,
        }))
    }

    fn static_method_marker(&mut self) -> bool {
        if matches!(&self.peek().kind, TokenKind::Ident(name) if name == "static")
            && matches!(
                self.tokens.get(self.pos + 1).map(|t| &t.kind),
                Some(TokenKind::Ident(_))
            )
        {
            self.advance();
            true
        } else {
            false
        }
    }

    fn accessor_marker(&mut self) -> Option<&'static str> {
        if matches!(&self.peek().kind, TokenKind::Ident(name) if name == "get" || name == "set")
            && !matches!(
                self.tokens.get(self.pos + 1).map(|t| &t.kind),
                Some(TokenKind::LParen)
            )
        {
            let marker = if matches!(&self.peek().kind, TokenKind::Ident(name) if name == "get") {
                "__get"
            } else {
                "__set"
            };
            self.advance();
            Some(marker)
        } else {
            None
        }
    }

    fn async_method_marker(&mut self) {
        if matches!(&self.peek().kind, TokenKind::Ident(name) if name == "async")
            && !matches!(
                self.tokens.get(self.pos + 1).map(|t| &t.kind),
                Some(TokenKind::LParen)
            )
        {
            self.advance();
        }
    }

    fn method_name(&mut self) -> Result<String, String> {
        if let TokenKind::Number(n) = self.peek().kind.clone() {
            self.advance();
            Ok(JsValue::Number(n).display())
        } else if self.check(&TokenKind::LBracket) {
            self.computed_property_name()
        } else {
            self.consume_property("Expected method name")
        }
    }

    fn params(&mut self) -> Result<(Vec<String>, Vec<Stmt>), String> {
        let mut params = Vec::new();
        let mut defaults = Vec::new();
        if !self.check(&TokenKind::RParen) {
            loop {
                let rest = self.matches(&[TokenKind::Spread]);
                if matches!(self.peek().kind, TokenKind::LBrace) {
                    let temp = format!("__param{}", params.len());
                    let bindings = self.object_binding_pattern()?;
                    if self.matches(&[TokenKind::Eq]) {
                        defaults.push(default_param_stmt(temp.clone(), self.expression()?));
                    }
                    defaults.push(Stmt::Destructure(bindings, Expr::Var(temp.clone())));
                    params.push(temp);
                    if !self.matches(&[TokenKind::Comma]) {
                        break;
                    }
                    continue;
                }
                if matches!(self.peek().kind, TokenKind::LBracket) {
                    let temp = format!("__param{}", params.len());
                    let bindings = self.array_binding_pattern()?;
                    if self.matches(&[TokenKind::Eq]) {
                        defaults.push(default_param_stmt(temp.clone(), self.expression()?));
                    }
                    defaults.push(Stmt::ArrayDestructure(bindings, Expr::Var(temp.clone())));
                    params.push(temp);
                    if !self.matches(&[TokenKind::Comma]) {
                        break;
                    }
                    continue;
                }
                let name = self.consume_ident("Expected parameter")?;
                if !rest && self.matches(&[TokenKind::Eq]) {
                    defaults.push(default_param_stmt(name.clone(), self.expression()?));
                }
                params.push(if rest { format!("...{name}") } else { name });
                if !self.matches(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        self.consume(&TokenKind::RParen, "Expected ')' after parameters")?;
        Ok((params, defaults))
    }

    fn block(&mut self) -> Result<Vec<Stmt>, String> {
        let mut out = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_eof() {
            out.push(self.statement()?);
        }
        self.consume(&TokenKind::RBrace, "Expected '}'")?;
        Ok(out)
    }
    fn if_stmt(&mut self) -> Result<Stmt, String> {
        self.consume(&TokenKind::LParen, "Expected '('")?;
        let cond = self.comma_expression()?;
        self.consume(&TokenKind::RParen, "Expected ')'")?;
        let then = Box::new(self.statement()?);
        let els = if self.matches(&[TokenKind::Else]) {
            Some(Box::new(self.statement()?))
        } else {
            None
        };
        Ok(Stmt::If(cond, then, els))
    }
    fn while_stmt(&mut self) -> Result<Stmt, String> {
        self.consume(&TokenKind::LParen, "Expected '('")?;
        let cond = self.comma_expression()?;
        self.consume(&TokenKind::RParen, "Expected ')'")?;
        Ok(Stmt::While(cond, Box::new(self.statement()?)))
    }

    fn do_while_stmt(&mut self) -> Result<Stmt, String> {
        let body = Box::new(self.statement()?);
        self.consume(&TokenKind::While, "Expected while after do body")?;
        self.consume(&TokenKind::LParen, "Expected '(' after while")?;
        let cond = self.comma_expression()?;
        self.consume(&TokenKind::RParen, "Expected ')' after condition")?;
        self.consume_optional_semi();
        Ok(Stmt::DoWhile(body, cond))
    }

    fn try_stmt(&mut self) -> Result<Stmt, String> {
        self.consume(&TokenKind::LBrace, "Expected try body")?;
        let body = self.block()?;
        let catch = if self.matches(&[TokenKind::Catch]) {
            let name = if self.matches(&[TokenKind::LParen]) {
                let name = self.consume_ident("Expected catch binding")?;
                self.consume(&TokenKind::RParen, "Expected ')' after catch binding")?;
                name
            } else {
                "__catch".into()
            };
            self.consume(&TokenKind::LBrace, "Expected catch body")?;
            Some((name, self.block()?))
        } else {
            None
        };
        let finally = if self.matches(&[TokenKind::Finally]) {
            self.consume(&TokenKind::LBrace, "Expected finally body")?;
            Some(self.block()?)
        } else {
            None
        };
        Ok(Stmt::Try(body, catch, finally))
    }

    fn switch_stmt(&mut self) -> Result<Stmt, String> {
        self.consume(&TokenKind::LParen, "Expected '(' after switch")?;
        let value = self.comma_expression()?;
        self.consume(&TokenKind::RParen, "Expected ')' after switch value")?;
        self.consume(&TokenKind::LBrace, "Expected switch body")?;
        let mut cases = Vec::new();
        while !self.check(&TokenKind::RBrace) && !self.is_eof() {
            let label = if self.matches(&[TokenKind::Case]) {
                let value = self.expression()?;
                self.consume(&TokenKind::Colon, "Expected ':' after case")?;
                Some(value)
            } else {
                self.consume(&TokenKind::Default, "Expected case or default")?;
                self.consume(&TokenKind::Colon, "Expected ':' after default")?;
                None
            };
            let mut body = Vec::new();
            while !matches!(self.peek().kind, TokenKind::Case | TokenKind::Default)
                && !self.check(&TokenKind::RBrace)
                && !self.is_eof()
            {
                body.push(self.statement()?);
            }
            cases.push((label, body));
        }
        self.consume(&TokenKind::RBrace, "Expected switch body end")?;
        Ok(Stmt::Switch(value, cases))
    }

    fn for_stmt(&mut self) -> Result<Stmt, String> {
        if matches!(&self.peek().kind, TokenKind::Ident(name) if name == "await") {
            self.advance();
        }
        self.consume(&TokenKind::LParen, "Expected '(' after for")?;
        let init = if self.matches(&[TokenKind::Semi]) {
            None
        } else if self.matches(&[TokenKind::Let, TokenKind::Const, TokenKind::Var]) {
            if matches!(self.peek().kind, TokenKind::LBracket) {
                let bindings = self.array_binding_pattern()?;
                if self.matches(&[TokenKind::Of]) {
                    return self.for_of_array_destructure_stmt(bindings);
                }
                return Err(format!(
                    "Expected 'of' after array for binding at {}:{}",
                    self.peek().line,
                    self.peek().col
                ));
            }
            if matches!(self.peek().kind, TokenKind::LBrace) {
                let bindings = self.object_binding_pattern()?;
                if self.matches(&[TokenKind::Of]) {
                    return self.for_of_object_destructure_stmt(bindings);
                }
                return Err(format!(
                    "Expected 'of' after object for binding at {}:{}",
                    self.peek().line,
                    self.peek().col
                ));
            }
            let name = self.consume_ident("Expected variable name")?;
            if self.matches(&[TokenKind::In]) {
                return self.for_in_stmt(name);
            }
            if self.matches(&[TokenKind::Of]) {
                return self.for_of_stmt(name);
            }
            let mut vars = vec![(name, self.var_initializer()?)];
            while self.matches(&[TokenKind::Comma]) {
                let name = self.consume_ident("Expected variable name")?;
                vars.push((name, self.var_initializer()?));
            }
            self.consume(&TokenKind::Semi, "Expected ';' after for initializer")?;
            Some(Box::new(if vars.len() == 1 {
                let (name, init) = vars.pop().unwrap();
                Stmt::Var(name, init)
            } else {
                Stmt::Vars(vars)
            }))
        } else if let Some(name) = self.for_in_binding() {
            return self.for_in_stmt(name);
        } else if let Some(name) = self.for_of_binding() {
            return self.for_of_stmt(name);
        } else {
            let expr = self.comma_expression()?;
            self.consume(&TokenKind::Semi, "Expected ';' after for initializer")?;
            Some(Box::new(Stmt::Expr(expr)))
        };
        let condition = if self.check(&TokenKind::Semi) {
            None
        } else {
            Some(self.comma_expression()?)
        };
        self.consume(&TokenKind::Semi, "Expected ';' after for condition")?;
        let increment = if self.check(&TokenKind::RParen) {
            None
        } else {
            Some(self.comma_expression()?)
        };
        self.consume(&TokenKind::RParen, "Expected ')' after for clauses")?;
        Ok(Stmt::For(
            init,
            condition,
            increment,
            Box::new(self.statement()?),
        ))
    }

    fn for_in_binding(&mut self) -> Option<String> {
        let TokenKind::Ident(name) = self.peek().kind.clone() else {
            return None;
        };
        if !matches!(
            self.tokens.get(self.pos + 1).map(|t| &t.kind),
            Some(TokenKind::In)
        ) {
            return None;
        }
        self.advance();
        self.advance();
        Some(name)
    }

    fn for_of_array_destructure_stmt(
        &mut self,
        bindings: Vec<(String, String, Option<Expr>, bool)>,
    ) -> Result<Stmt, String> {
        let temp = format!("__for_item{}", self.pos);
        let source = self.comma_expression()?;
        self.consume(&TokenKind::RParen, "Expected ')' after for-of source")?;
        let body = self.statement()?;
        Ok(Stmt::ForOf(
            temp.clone(),
            source,
            Box::new(Stmt::Many(vec![
                Stmt::ArrayDestructure(bindings, Expr::Var(temp)),
                body,
            ])),
        ))
    }

    fn for_of_object_destructure_stmt(
        &mut self,
        bindings: Vec<(String, String, Option<Expr>)>,
    ) -> Result<Stmt, String> {
        let temp = format!("__for_item{}", self.pos);
        let source = self.comma_expression()?;
        self.consume(&TokenKind::RParen, "Expected ')' after for-of source")?;
        let body = self.statement()?;
        Ok(Stmt::ForOf(
            temp.clone(),
            source,
            Box::new(Stmt::Many(vec![
                Stmt::Destructure(bindings, Expr::Var(temp)),
                body,
            ])),
        ))
    }

    fn var_initializer(&mut self) -> Result<Option<Expr>, String> {
        if self.matches(&[TokenKind::Eq]) {
            Ok(Some(self.expression()?))
        } else {
            Ok(None)
        }
    }

    fn for_of_binding(&mut self) -> Option<String> {
        let TokenKind::Ident(name) = self.peek().kind.clone() else {
            return None;
        };
        if !matches!(
            self.tokens.get(self.pos + 1).map(|t| &t.kind),
            Some(TokenKind::Of)
        ) {
            return None;
        }
        self.advance();
        self.advance();
        Some(name)
    }

    fn for_in_stmt(&mut self, name: String) -> Result<Stmt, String> {
        let iterable = self.comma_expression()?;
        self.consume(&TokenKind::RParen, "Expected ')' after for-in source")?;
        Ok(Stmt::ForIn(name, iterable, Box::new(self.statement()?)))
    }

    fn for_of_stmt(&mut self, name: String) -> Result<Stmt, String> {
        let iterable = self.comma_expression()?;
        self.consume(&TokenKind::RParen, "Expected ')' after for-of source")?;
        Ok(Stmt::ForOf(name, iterable, Box::new(self.statement()?)))
    }

    fn expression(&mut self) -> Result<Expr, String> {
        self.assignment()
    }

    fn comma_expression(&mut self) -> Result<Expr, String> {
        let first = self.assignment()?;
        if !self.matches(&[TokenKind::Comma]) {
            return Ok(first);
        }
        let mut items = vec![first];
        loop {
            items.push(self.assignment()?);
            if !self.matches(&[TokenKind::Comma]) {
                return Ok(Expr::Sequence(items));
            }
        }
    }

    fn assignment(&mut self) -> Result<Expr, String> {
        if self.starts_arrow_function() {
            let (params, defaults) = self.parse_arrow_params()?;
            return self.arrow_body(params, defaults);
        }
        let expr = self.conditional()?;
        if self.matches(&[TokenKind::Eq]) {
            Ok(Expr::Assign(Box::new(expr), Box::new(self.assignment()?)))
        } else if let Some(op) = self.compound_assignment() {
            Ok(Expr::AssignOp(
                Box::new(expr),
                op.into(),
                Box::new(self.assignment()?),
            ))
        } else {
            Ok(expr)
        }
    }

    fn compound_assignment(&mut self) -> Option<&'static str> {
        if self.matches(&[TokenKind::PlusEq]) {
            Some("+")
        } else if self.matches(&[TokenKind::MinusEq]) {
            Some("-")
        } else if self.matches(&[TokenKind::StarEq]) {
            Some("*")
        } else if self.matches(&[TokenKind::SlashEq]) {
            Some("/")
        } else if self.matches(&[TokenKind::PercentEq]) {
            Some("%")
        } else if self.matches(&[TokenKind::ShlEq]) {
            Some("<<")
        } else if self.matches(&[TokenKind::ShrEq]) {
            Some(">>")
        } else if self.matches(&[TokenKind::UShrEq]) {
            Some(">>>")
        } else if self.matches(&[TokenKind::BitAndEq]) {
            Some("&")
        } else if self.matches(&[TokenKind::BitOrEq]) {
            Some("|")
        } else if self.matches(&[TokenKind::CaretEq]) {
            Some("^")
        } else {
            None
        }
    }

    fn starts_arrow_function(&self) -> bool {
        let start = if matches!(&self.peek().kind, TokenKind::Ident(name) if name == "async") {
            self.pos + 1
        } else {
            self.pos
        };
        match self.tokens.get(start).map(|token| &token.kind) {
            Some(TokenKind::Ident(_)) => matches!(
                self.tokens.get(start + 1).map(|token| &token.kind),
                Some(TokenKind::FatArrow)
            ),
            Some(TokenKind::LParen) => self.matching_rparen(start).is_some_and(|end| {
                matches!(
                    self.tokens.get(end + 1).map(|token| &token.kind),
                    Some(TokenKind::FatArrow)
                )
            }),
            _ => false,
        }
    }

    fn matching_rparen(&self, start: usize) -> Option<usize> {
        let mut depth = 0usize;
        for index in start..self.tokens.len() {
            match self.tokens.get(index).map(|token| &token.kind) {
                Some(TokenKind::LParen) => depth += 1,
                Some(TokenKind::RParen) => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        return Some(index);
                    }
                }
                Some(TokenKind::Eof) | None => return None,
                _ => {}
            }
        }
        None
    }

    fn parse_arrow_params(&mut self) -> Result<(Vec<String>, Vec<Stmt>), String> {
        if matches!(&self.peek().kind, TokenKind::Ident(name) if name == "async") {
            self.advance();
        }
        if let TokenKind::Ident(name) = self.peek().kind.clone() {
            self.advance();
            self.consume(&TokenKind::FatArrow, "Expected arrow after parameter")?;
            return Ok((vec![name], Vec::new()));
        }
        self.consume(&TokenKind::LParen, "Expected arrow parameters")?;
        let parsed = self.params()?;
        self.consume(&TokenKind::FatArrow, "Expected arrow after parameters")?;
        Ok(parsed)
    }

    fn arrow_body(&mut self, params: Vec<String>, defaults: Vec<Stmt>) -> Result<Expr, String> {
        if self.matches(&[TokenKind::LBrace]) {
            Ok(Expr::Function(
                None,
                params,
                with_param_defaults(defaults, self.block()?),
            ))
        } else {
            Ok(Expr::Function(
                None,
                params,
                with_param_defaults(defaults, vec![Stmt::Return(Some(self.assignment()?))]),
            ))
        }
    }
    fn conditional(&mut self) -> Result<Expr, String> {
        let expr = self.nullish()?;
        if !self.matches(&[TokenKind::Question]) {
            return Ok(expr);
        }
        let then_expr = self.expression()?;
        self.consume(&TokenKind::Colon, "Expected ':' in conditional expression")?;
        let else_expr = self.assignment()?;
        Ok(Expr::Conditional(
            Box::new(expr),
            Box::new(then_expr),
            Box::new(else_expr),
        ))
    }
    fn nullish(&mut self) -> Result<Expr, String> {
        let mut e = self.or()?;
        while self.matches(&[TokenKind::Nullish]) {
            e = Expr::Binary(Box::new(e), "??".into(), Box::new(self.or()?));
        }
        Ok(e)
    }
    fn or(&mut self) -> Result<Expr, String> {
        let mut e = self.and()?;
        while self.matches(&[TokenKind::OrOr]) {
            e = Expr::Binary(Box::new(e), "||".into(), Box::new(self.and()?));
        }
        Ok(e)
    }
    fn and(&mut self) -> Result<Expr, String> {
        let mut e = self.bit_or()?;
        while self.matches(&[TokenKind::AndAnd]) {
            e = Expr::Binary(Box::new(e), "&&".into(), Box::new(self.bit_or()?));
        }
        Ok(e)
    }
    fn bit_or(&mut self) -> Result<Expr, String> {
        let mut e = self.bit_xor()?;
        while self.matches(&[TokenKind::BitOr]) {
            e = Expr::Binary(Box::new(e), "|".into(), Box::new(self.bit_xor()?));
        }
        Ok(e)
    }
    fn bit_xor(&mut self) -> Result<Expr, String> {
        let mut e = self.bit_and()?;
        while self.matches(&[TokenKind::Caret]) {
            e = Expr::Binary(Box::new(e), "^".into(), Box::new(self.bit_and()?));
        }
        Ok(e)
    }
    fn bit_and(&mut self) -> Result<Expr, String> {
        let mut e = self.equality()?;
        while self.matches(&[TokenKind::BitAnd]) {
            e = Expr::Binary(Box::new(e), "&".into(), Box::new(self.equality()?));
        }
        Ok(e)
    }
    fn equality(&mut self) -> Result<Expr, String> {
        let mut e = self.comparison()?;
        while self.matches(&[
            TokenKind::EqEq,
            TokenKind::BangEq,
            TokenKind::StrictEq,
            TokenKind::StrictBangEq,
        ]) {
            let op = match &self.previous().kind {
                TokenKind::EqEq => "==",
                TokenKind::BangEq => "!=",
                TokenKind::StrictEq => "===",
                _ => "!==",
            };
            e = Expr::Binary(Box::new(e), op.into(), Box::new(self.comparison()?));
        }
        Ok(e)
    }
    fn comparison(&mut self) -> Result<Expr, String> {
        let mut e = self.shift()?;
        while self.matches(&[
            TokenKind::Lt,
            TokenKind::Lte,
            TokenKind::Gt,
            TokenKind::Gte,
            TokenKind::In,
            TokenKind::Instanceof,
        ]) {
            let op = match &self.previous().kind {
                TokenKind::Lt => "<",
                TokenKind::Lte => "<=",
                TokenKind::Gt => ">",
                TokenKind::Gte => ">=",
                TokenKind::In => "in",
                _ => "instanceof",
            };
            e = Expr::Binary(Box::new(e), op.into(), Box::new(self.shift()?));
        }
        Ok(e)
    }
    fn shift(&mut self) -> Result<Expr, String> {
        let mut e = self.term()?;
        while self.matches(&[TokenKind::Shl, TokenKind::Shr, TokenKind::UShr]) {
            let op = match &self.previous().kind {
                TokenKind::Shl => "<<",
                TokenKind::Shr => ">>",
                _ => ">>>",
            };
            e = Expr::Binary(Box::new(e), op.into(), Box::new(self.term()?));
        }
        Ok(e)
    }
    fn term(&mut self) -> Result<Expr, String> {
        let mut e = self.factor()?;
        while self.matches(&[TokenKind::Plus, TokenKind::Minus]) {
            let op = if self.previous().kind == TokenKind::Plus {
                "+"
            } else {
                "-"
            };
            e = Expr::Binary(Box::new(e), op.into(), Box::new(self.factor()?));
        }
        Ok(e)
    }
    fn factor(&mut self) -> Result<Expr, String> {
        let mut e = self.power()?;
        while self.matches(&[TokenKind::Star, TokenKind::Slash, TokenKind::Percent]) {
            let op = match &self.previous().kind {
                TokenKind::Star => "*",
                TokenKind::Slash => "/",
                _ => "%",
            };
            e = Expr::Binary(Box::new(e), op.into(), Box::new(self.power()?));
        }
        Ok(e)
    }

    fn power(&mut self) -> Result<Expr, String> {
        let left = self.unary()?;
        if self.matches(&[TokenKind::StarStar]) {
            Ok(Expr::Binary(
                Box::new(left),
                "**".into(),
                Box::new(self.power()?),
            ))
        } else {
            Ok(left)
        }
    }

    fn unary(&mut self) -> Result<Expr, String> {
        if matches!(&self.peek().kind, TokenKind::Ident(name) if name == "await") {
            self.advance();
            return Ok(Expr::Await(Box::new(self.unary()?)));
        }
        if matches!(&self.peek().kind, TokenKind::Ident(name) if name == "yield") {
            self.advance();
            self.matches(&[TokenKind::Star]);
            if matches!(
                self.peek().kind,
                TokenKind::Semi | TokenKind::RParen | TokenKind::RBrace
            ) {
                return Ok(Expr::Literal(JsValue::Undefined));
            }
            return self.unary();
        }
        if self.matches(&[TokenKind::PlusPlus, TokenKind::MinusMinus]) {
            let delta = if self.previous().kind == TokenKind::PlusPlus {
                1
            } else {
                -1
            };
            return Ok(Expr::Update(Box::new(self.unary()?), delta, true));
        }
        if self.matches(&[
            TokenKind::Bang,
            TokenKind::Minus,
            TokenKind::Plus,
            TokenKind::Tilde,
        ]) {
            let op = match &self.previous().kind {
                TokenKind::Bang => "!",
                TokenKind::Minus => "-",
                TokenKind::Plus => "+",
                _ => "~",
            };
            Ok(Expr::Unary(op.into(), Box::new(self.unary()?)))
        } else if self.matches(&[TokenKind::Typeof]) {
            Ok(Expr::Typeof(Box::new(self.unary()?)))
        } else if self.matches(&[TokenKind::Void]) {
            self.unary()?;
            Ok(Expr::Literal(JsValue::Undefined))
        } else if self.matches(&[TokenKind::Delete]) {
            Ok(Expr::Delete(Box::new(self.unary()?)))
        } else if self.matches(&[TokenKind::New]) {
            self.new_expr()
        } else {
            self.call()
        }
    }

    fn new_expr(&mut self) -> Result<Expr, String> {
        let mut callee = self.primary()?;
        loop {
            if self.matches(&[TokenKind::Dot]) {
                callee = Expr::Get(
                    Box::new(callee),
                    self.consume_property("Expected property")?,
                );
            } else if self.matches(&[TokenKind::LBracket]) {
                let idx = self.expression()?;
                self.consume(&TokenKind::RBracket, "Expected ']'")?;
                callee = Expr::Index(Box::new(callee), Box::new(idx));
            } else {
                break;
            }
        }
        let args = if self.matches(&[TokenKind::LParen]) {
            self.args()?
        } else {
            Vec::new()
        };
        let mut expr = Expr::New(Box::new(callee), args);
        loop {
            if self.matches(&[TokenKind::LParen]) {
                expr = Expr::Call(Box::new(expr), self.args()?);
            } else if self.matches(&[TokenKind::Dot]) {
                expr = Expr::Get(Box::new(expr), self.consume_property("Expected property")?);
            } else if self.matches(&[TokenKind::OptionalChain]) {
                expr = self.optional_chain(expr)?;
            } else if self.matches(&[TokenKind::LBracket]) {
                let idx = self.expression()?;
                self.consume(&TokenKind::RBracket, "Expected ']'")?;
                expr = Expr::Index(Box::new(expr), Box::new(idx));
            } else {
                break;
            }
        }
        Ok(expr)
    }

    fn call(&mut self) -> Result<Expr, String> {
        let mut e = self.primary()?;
        loop {
            if self.matches(&[TokenKind::LParen]) {
                e = Expr::Call(Box::new(e), self.args()?);
            } else if self.matches(&[TokenKind::Dot]) {
                e = Expr::Get(Box::new(e), self.consume_property("Expected property")?);
            } else if self.matches(&[TokenKind::OptionalChain]) {
                e = self.optional_chain(e)?;
            } else if self.matches(&[TokenKind::LBracket]) {
                let idx = self.expression()?;
                self.consume(&TokenKind::RBracket, "Expected ']'")?;
                e = Expr::Index(Box::new(e), Box::new(idx));
            } else if self.matches(&[TokenKind::PlusPlus, TokenKind::MinusMinus]) {
                let delta = if self.previous().kind == TokenKind::PlusPlus {
                    1
                } else {
                    -1
                };
                e = Expr::Update(Box::new(e), delta, false);
            } else {
                break;
            }
        }
        Ok(e)
    }

    fn optional_chain(&mut self, base: Expr) -> Result<Expr, String> {
        if self.matches(&[TokenKind::LParen]) {
            return Ok(Expr::OptionalCall(Box::new(base), self.args()?));
        }
        if self.matches(&[TokenKind::LBracket]) {
            let idx = self.expression()?;
            self.consume(&TokenKind::RBracket, "Expected ']'")?;
            return Ok(Expr::OptionalIndex(Box::new(base), Box::new(idx)));
        }
        Ok(Expr::OptionalGet(
            Box::new(base),
            self.consume_property("Expected property")?,
        ))
    }

    fn args(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();
        if !self.check(&TokenKind::RParen) {
            loop {
                args.push(self.expression()?);
                if !self.matches(&[TokenKind::Comma]) {
                    break;
                }
            }
        }
        self.consume(&TokenKind::RParen, "Expected ')' after arguments")?;
        Ok(args)
    }

    fn primary(&mut self) -> Result<Expr, String> {
        let tok = self.advance().clone();
        match tok.kind {
            TokenKind::Number(n) => Ok(Expr::Literal(JsValue::Number(n))),
            TokenKind::String(s) => Ok(Expr::Literal(JsValue::String(s))),
            TokenKind::True => Ok(Expr::Literal(JsValue::Bool(true))),
            TokenKind::False => Ok(Expr::Literal(JsValue::Bool(false))),
            TokenKind::Null => Ok(Expr::Literal(JsValue::Null)),
            TokenKind::Ident(s) if s == "async" && self.matches(&[TokenKind::Function]) => {
                self.function_expr()
            }
            TokenKind::Ident(s) => Ok(Expr::Var(s)),
            TokenKind::Of => Ok(Expr::Var("of".into())),
            TokenKind::This => Ok(Expr::This),
            TokenKind::Class => {
                let name = if matches!(self.peek().kind, TokenKind::Ident(_)) {
                    Some(self.consume_ident("Expected class name")?)
                } else {
                    None
                };
                self.class_expr(name)
            }
            TokenKind::Function => self.function_expr(),
            TokenKind::LParen => {
                let mut items = vec![self.expression()?];
                while self.matches(&[TokenKind::Comma]) {
                    items.push(self.expression()?);
                }
                self.consume(&TokenKind::RParen, "Expected ')'")?;
                if items.len() == 1 {
                    Ok(items.pop().unwrap())
                } else {
                    Ok(Expr::Sequence(items))
                }
            }
            TokenKind::LBracket => {
                let mut items = Vec::new();
                while !self.check(&TokenKind::RBracket) {
                    if self.matches(&[TokenKind::Comma]) {
                        items.push(Expr::Literal(JsValue::Undefined));
                        continue;
                    }
                    let spread = self.matches(&[TokenKind::Spread]);
                    let item = self.expression()?;
                    items.push(if spread {
                        Expr::Spread(Box::new(item))
                    } else {
                        item
                    });
                    if !self.matches(&[TokenKind::Comma]) {
                        break;
                    }
                }
                self.consume(&TokenKind::RBracket, "Expected ']'")?;
                Ok(Expr::Array(items))
            }
            TokenKind::LBrace => {
                let mut props = Vec::new();
                if !self.check(&TokenKind::RBrace) {
                    loop {
                        if self.matches(&[TokenKind::Spread]) {
                            props.push((
                                format!("__spread{}", props.len()),
                                Expr::Spread(Box::new(self.expression()?)),
                            ));
                        } else {
                            props.push(self.object_prop()?);
                        }
                        if !self.matches(&[TokenKind::Comma]) {
                            break;
                        }
                    }
                }
                self.consume(&TokenKind::RBrace, "Expected '}'")?;
                Ok(Expr::Object(props))
            }
            TokenKind::Spread => Ok(Expr::Spread(Box::new(self.expression()?))),
            other => Err(format!(
                "Expected expression at {}:{}, got {:?} after {:?} before {:?}",
                tok.line,
                tok.col,
                other,
                self.tokens
                    .get(self.pos.saturating_sub(2))
                    .map(|token| &token.kind),
                self.tokens.get(self.pos).map(|token| &token.kind)
            )),
        }
    }

    fn object_prop(&mut self) -> Result<(String, Expr), String> {
        if self.matches(&[TokenKind::Star]) {
            let key = self.object_key()?;
            return self.object_method(key);
        }
        let key = self.object_key()?;
        if key == "async" && !self.check(&TokenKind::LParen) && !self.check(&TokenKind::Colon) {
            let name = self.object_key()?;
            return self.object_method(name);
        }
        if matches!(key.as_str(), "get" | "set")
            && matches!(self.peek().kind, TokenKind::Ident(_) | TokenKind::String(_))
        {
            let name = self.object_key()?;
            return self.object_method(format!("__{}:{name}", key));
        }
        if self.matches(&[TokenKind::LParen]) {
            return self.object_method_after_lparen(key);
        }
        if self.matches(&[TokenKind::Colon]) {
            return Ok((key, self.expression()?));
        }
        Ok((key.clone(), Expr::Var(key)))
    }

    fn object_key(&mut self) -> Result<String, String> {
        if let TokenKind::Number(n) = self.peek().kind.clone() {
            self.advance();
            Ok(JsValue::Number(n).display())
        } else if self.check(&TokenKind::LBracket) {
            self.computed_property_name()
        } else {
            self.consume_property("Expected object key")
        }
    }

    fn computed_property_name(&mut self) -> Result<String, String> {
        self.consume(&TokenKind::LBracket, "Expected computed property start")?;
        let expr = self.expression()?;
        self.consume(&TokenKind::RBracket, "Expected computed property end")?;
        Ok(static_property_name(&expr))
    }

    fn object_method(&mut self, key: String) -> Result<(String, Expr), String> {
        self.consume(&TokenKind::LParen, "Expected method parameters")?;
        self.object_method_after_lparen(key)
    }

    fn object_method_after_lparen(&mut self, key: String) -> Result<(String, Expr), String> {
        let (params, defaults) = self.params()?;
        self.consume(&TokenKind::LBrace, "Expected method body")?;
        Ok((
            key,
            Expr::Function(None, params, with_param_defaults(defaults, self.block()?)),
        ))
    }

    fn function_expr(&mut self) -> Result<Expr, String> {
        self.matches(&[TokenKind::Star]);
        let name = if matches!(self.peek().kind, TokenKind::Ident(_)) {
            Some(self.consume_ident("Expected function name")?)
        } else {
            None
        };
        self.consume(&TokenKind::LParen, "Expected '(' after function")?;
        let (params, defaults) = self.params()?;
        self.consume(&TokenKind::LBrace, "Expected function body")?;
        Ok(Expr::Function(
            name,
            params,
            with_param_defaults(defaults, self.block()?),
        ))
    }

    fn matches(&mut self, kinds: &[TokenKind]) -> bool {
        for k in kinds {
            if self.check(k) {
                self.advance();
                return true;
            }
        }
        false
    }
    fn check(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.peek().kind) == std::mem::discriminant(kind)
    }
    fn consume(&mut self, kind: &TokenKind, msg: &str) -> Result<(), String> {
        if self.check(kind) {
            self.advance();
            Ok(())
        } else {
            Err(format!(
                "{} at {}:{}",
                msg,
                self.peek().line,
                self.peek().col
            ))
        }
    }
    fn consume_ident(&mut self, msg: &str) -> Result<String, String> {
        match self.advance().kind.clone() {
            TokenKind::Ident(s) => Ok(s),
            TokenKind::Of => Ok("of".into()),
            other => Err(format!(
                "{} at {}:{}, got {:?}",
                msg,
                self.previous().line,
                self.previous().col,
                other
            )),
        }
    }
    fn consume_property(&mut self, msg: &str) -> Result<String, String> {
        match self.advance().kind.clone() {
            TokenKind::Ident(s) | TokenKind::String(s) => Ok(s),
            TokenKind::Class => Ok("class".into()),
            TokenKind::Break => Ok("break".into()),
            TokenKind::Case => Ok("case".into()),
            TokenKind::Catch => Ok("catch".into()),
            TokenKind::Continue => Ok("continue".into()),
            TokenKind::Default => Ok("default".into()),
            TokenKind::Delete => Ok("delete".into()),
            TokenKind::Do => Ok("do".into()),
            TokenKind::Else => Ok("else".into()),
            TokenKind::False => Ok("false".into()),
            TokenKind::For => Ok("for".into()),
            TokenKind::Finally => Ok("finally".into()),
            TokenKind::Function => Ok("function".into()),
            TokenKind::If => Ok("if".into()),
            TokenKind::In => Ok("in".into()),
            TokenKind::Instanceof => Ok("instanceof".into()),
            TokenKind::New => Ok("new".into()),
            TokenKind::Null => Ok("null".into()),
            TokenKind::Of => Ok("of".into()),
            TokenKind::Return => Ok("return".into()),
            TokenKind::Switch => Ok("switch".into()),
            TokenKind::This => Ok("this".into()),
            TokenKind::Throw => Ok("throw".into()),
            TokenKind::True => Ok("true".into()),
            TokenKind::Try => Ok("try".into()),
            TokenKind::Typeof => Ok("typeof".into()),
            TokenKind::Void => Ok("void".into()),
            TokenKind::While => Ok("while".into()),
            other => Err(format!(
                "{} at {}:{}, got {:?}",
                msg,
                self.previous().line,
                self.previous().col,
                other
            )),
        }
    }
    fn consume_optional_semi(&mut self) {
        self.matches(&[TokenKind::Semi]);
    }
    fn consume_optional_label(&mut self) -> Option<String> {
        let TokenKind::Ident(name) = self.peek().kind.clone() else {
            return None;
        };
        self.advance();
        Some(name)
    }
    fn label_name(&mut self) -> Option<String> {
        let TokenKind::Ident(name) = self.peek().kind.clone() else {
            return None;
        };
        if !matches!(
            self.tokens.get(self.pos + 1).map(|t| &t.kind),
            Some(TokenKind::Colon)
        ) {
            return None;
        }
        self.advance();
        self.advance();
        Some(name)
    }
    fn is_eof(&self) -> bool {
        self.check(&TokenKind::Eof)
    }
    fn peek(&self) -> &Token {
        &self.tokens[self.pos]
    }
    fn previous(&self) -> &Token {
        &self.tokens[self.pos - 1]
    }
    fn advance(&mut self) -> &Token {
        if !self.is_eof() {
            self.pos += 1;
            js_trace::parser_progress(self.pos, self.tokens.len(), &self.previous().kind);
        }
        self.previous()
    }
}

enum Flow {
    Value(JsValue),
    Return(JsValue),
    Break(Option<String>),
    Continue(Option<String>),
}

fn execute_block(stmts: &[Stmt], env: EnvRef) -> Result<Flow, String> {
    hoist_declarations(stmts, env.clone());
    let mut last = JsValue::Undefined;
    for stmt in stmts {
        match execute(stmt, env.clone())? {
            Flow::Value(v) => last = v,
            other => return Ok(other),
        }
    }
    Ok(Flow::Value(last))
}

fn hoist_declarations(stmts: &[Stmt], env: EnvRef) {
    for stmt in stmts {
        hoist_vars(stmt, env.clone());
    }
    hoist_functions(stmts, env);
}

fn hoist_vars(stmt: &Stmt, env: EnvRef) {
    match stmt {
        Stmt::Var(name, _) => hoist_name(name, env),
        Stmt::Vars(vars) => {
            for (name, _) in vars {
                hoist_name(name, env.clone());
            }
        }
        Stmt::Destructure(bindings, _) => {
            for (_, name, _) in bindings {
                hoist_name(name, env.clone());
            }
        }
        Stmt::ArrayDestructure(bindings, _) => {
            for (_, name, _, _) in bindings {
                hoist_name(name, env.clone());
            }
        }
        Stmt::Many(stmts) | Stmt::Block(stmts) => {
            for stmt in stmts {
                hoist_vars(stmt, env.clone());
            }
        }
        Stmt::Label(_, stmt) => hoist_vars(stmt, env),
        Stmt::If(_, then_branch, else_branch) => {
            hoist_vars(then_branch, env.clone());
            if let Some(else_branch) = else_branch {
                hoist_vars(else_branch, env);
            }
        }
        Stmt::Switch(_, cases) => {
            for (_, stmts) in cases {
                for stmt in stmts {
                    hoist_vars(stmt, env.clone());
                }
            }
        }
        Stmt::While(_, body) | Stmt::DoWhile(body, _) => hoist_vars(body, env),
        Stmt::For(init, _, _, body) => {
            if let Some(init) = init {
                hoist_vars(init, env.clone());
            }
            hoist_vars(body, env);
        }
        Stmt::ForIn(name, _, body) | Stmt::ForOf(name, _, body) => {
            hoist_name(name, env.clone());
            hoist_vars(body, env);
        }
        Stmt::Try(body, catch, finally) => {
            for stmt in body {
                hoist_vars(stmt, env.clone());
            }
            if let Some((_, stmts)) = catch {
                for stmt in stmts {
                    hoist_vars(stmt, env.clone());
                }
            }
            if let Some(stmts) = finally {
                for stmt in stmts {
                    hoist_vars(stmt, env.clone());
                }
            }
        }
        _ => {}
    }
}

fn hoist_name(name: &str, env: EnvRef) {
    if !env.borrow().values.contains_key(name) {
        env.borrow_mut().define(name, JsValue::Undefined);
    }
}

fn declare_binding(env: EnvRef, name: &str, value: JsValue) {
    if Env::assign(&env, name, value.clone()).is_err() {
        env.borrow_mut().define(name, value);
    }
}

fn hoist_functions(stmts: &[Stmt], env: EnvRef) {
    for stmt in stmts {
        if let Stmt::Function(name, params, body) = stmt {
            let function = function_value(Some(name.clone()), params, body, env.clone());
            env.borrow_mut().define(name, function);
        }
    }
}

fn function_value(name: Option<String>, params: &[String], body: &[Stmt], env: EnvRef) -> JsValue {
    let mut properties = HashMap::new();
    properties.insert(
        "prototype".into(),
        JsValue::Object(Rc::new(RefCell::new(HashMap::new()))),
    );
    JsValue::Function(Rc::new(JsFunction {
        name,
        params: params.to_vec(),
        body: body.to_vec(),
        env,
        superclass: None,
        properties: RefCell::new(properties),
    }))
}

fn stmt_group(mut stmts: Vec<Stmt>) -> Stmt {
    if stmts.len() == 1 {
        stmts.pop().unwrap()
    } else {
        Stmt::Many(stmts)
    }
}

fn with_param_defaults(mut defaults: Vec<Stmt>, mut body: Vec<Stmt>) -> Vec<Stmt> {
    defaults.append(&mut body);
    defaults
}

fn accessor_name(accessor: Option<&str>, name: String) -> String {
    accessor.map_or(name.clone(), |accessor| format!("{accessor}:{name}"))
}

fn static_property_name(expr: &Expr) -> String {
    match expr {
        Expr::Literal(value) => value.display(),
        Expr::Var(name) => name.clone(),
        Expr::Get(base, prop) => format!("{}.{}", static_property_name(base), prop),
        _ => "__computed".into(),
    }
}

fn binding_path(prefix: &str, source: &str) -> String {
    if prefix.is_empty() {
        source.into()
    } else {
        format!("{prefix}.{source}")
    }
}

fn default_param_stmt(name: String, default: Expr) -> Stmt {
    Stmt::If(
        Expr::Binary(
            Box::new(Expr::Typeof(Box::new(Expr::Var(name.clone())))),
            "===".into(),
            Box::new(Expr::Literal(JsValue::String("undefined".into()))),
        ),
        Box::new(Stmt::Expr(Expr::Assign(
            Box::new(Expr::Var(name)),
            Box::new(default),
        ))),
        None,
    )
}

fn execute(stmt: &Stmt, env: EnvRef) -> Result<Flow, String> {
    js_budget::tick()?;
    match stmt {
        Stmt::Expr(e) => Ok(Flow::Value(eval_expr(e, env)?)),
        Stmt::Empty => Ok(Flow::Value(JsValue::Undefined)),
        Stmt::Many(stmts) => {
            let mut last = JsValue::Undefined;
            for stmt in stmts {
                match execute(stmt, env.clone())? {
                    Flow::Value(value) => last = value,
                    other => return Ok(other),
                }
            }
            Ok(Flow::Value(last))
        }
        Stmt::Var(name, init) => {
            let v = init
                .as_ref()
                .map(|e| eval_expr(e, env.clone()))
                .transpose()?
                .unwrap_or(JsValue::Undefined);
            declare_binding(env, name, v);
            Ok(Flow::Value(JsValue::Undefined))
        }
        Stmt::Vars(vars) => {
            for (name, init) in vars {
                let value = init
                    .as_ref()
                    .map(|e| eval_expr(e, env.clone()))
                    .transpose()?
                    .unwrap_or(JsValue::Undefined);
                declare_binding(env.clone(), name, value);
            }
            Ok(Flow::Value(JsValue::Undefined))
        }
        Stmt::Destructure(bindings, init) => {
            let source = eval_expr(init, env.clone())?;
            for (prop, name, default) in bindings {
                let mut value = if prop == "__rest" {
                    source.clone()
                } else {
                    get_binding_property(&source, prop)?
                };
                if matches!(value, JsValue::Undefined) {
                    if let Some(default) = default {
                        value = eval_expr(default, env.clone())?;
                    }
                }
                declare_binding(env.clone(), name, value);
            }
            Ok(Flow::Value(JsValue::Undefined))
        }
        Stmt::ArrayDestructure(bindings, init) => {
            let source = spread_values(eval_expr(init, env.clone())?);
            for (path, name, default, rest) in bindings {
                let mut value = if *rest {
                    let index = path.parse::<usize>().unwrap_or(0);
                    JsValue::Array(Rc::new(RefCell::new(
                        source.get(index..).unwrap_or(&[]).to_vec(),
                    )))
                } else {
                    get_array_binding_value(&source, path)?
                };
                if matches!(value, JsValue::Undefined) {
                    if let Some(default) = default {
                        value = eval_expr(default, env.clone())?;
                    }
                }
                declare_binding(env.clone(), name, value);
            }
            Ok(Flow::Value(JsValue::Undefined))
        }
        Stmt::Function(name, params, body) => {
            if !env.borrow().values.contains_key(name) {
                let function = function_value(Some(name.clone()), params, body, env.clone());
                env.borrow_mut().define(name, function);
            }
            Ok(Flow::Value(JsValue::Undefined))
        }
        Stmt::Return(e) => Ok(Flow::Return(
            e.as_ref()
                .map(|e| eval_expr(e, env))
                .transpose()?
                .unwrap_or(JsValue::Undefined),
        )),
        Stmt::Throw(e) => Err(format!("Uncaught {}", eval_expr(e, env)?.display())),
        Stmt::Block(stmts) => execute_block(stmts, env),
        Stmt::Label(label, stmt) => match execute(stmt, env)? {
            Flow::Break(Some(target)) if target == *label => Ok(Flow::Value(JsValue::Undefined)),
            Flow::Continue(Some(target)) if target == *label => Ok(Flow::Continue(None)),
            other => Ok(other),
        },
        Stmt::If(c, t, e) => {
            if eval_expr(c, env.clone())?.truthy() {
                execute(t, env)
            } else if let Some(e) = e {
                execute(e, env)
            } else {
                Ok(Flow::Value(JsValue::Undefined))
            }
        }
        Stmt::Switch(value, cases) => execute_switch(value, cases, env),
        Stmt::While(c, body) => {
            let mut last = JsValue::Undefined;
            while eval_expr(c, env.clone())?.truthy() {
                match execute(body, env.clone())? {
                    Flow::Value(v) => last = v,
                    Flow::Break(None) => break,
                    Flow::Continue(None) => continue,
                    other => return Ok(other),
                }
            }
            Ok(Flow::Value(last))
        }
        Stmt::DoWhile(body, cond) => execute_do_while(body, cond, env),
        Stmt::For(init, condition, increment, body) => execute_for(
            init.as_deref(),
            condition.as_ref(),
            increment.as_ref(),
            body,
            env,
        ),
        Stmt::ForIn(name, iterable, body) => execute_for_in(name, iterable, body, env),
        Stmt::ForOf(name, iterable, body) => execute_for_of(name, iterable, body, env),
        Stmt::Try(body, catch, finally) => execute_try(body, catch.as_ref(), finally.as_ref(), env),
        Stmt::Break(label) => Ok(Flow::Break(label.clone())),
        Stmt::Continue(label) => Ok(Flow::Continue(label.clone())),
    }
}

fn execute_switch(
    value: &Expr,
    cases: &[(Option<Expr>, Vec<Stmt>)],
    env: EnvRef,
) -> Result<Flow, String> {
    let value = eval_expr(value, env.clone())?;
    let start = matching_case(&value, cases, env.clone()).unwrap_or(cases.len());
    let mut last = JsValue::Undefined;
    for (_, body) in cases.iter().skip(start) {
        for stmt in body {
            match execute(stmt, env.clone())? {
                Flow::Value(v) => last = v,
                Flow::Break(None) => return Ok(Flow::Value(last)),
                other => return Ok(other),
            }
        }
    }
    Ok(Flow::Value(last))
}

fn matching_case(
    value: &JsValue,
    cases: &[(Option<Expr>, Vec<Stmt>)],
    env: EnvRef,
) -> Option<usize> {
    let mut default = None;
    for (index, (label, _)) in cases.iter().enumerate() {
        if let Some(label) = label {
            if eval_expr(label, env.clone()).ok().as_ref() == Some(value) {
                return Some(index);
            }
        } else {
            default = Some(index);
        }
    }
    default
}

fn execute_do_while(body: &Stmt, cond: &Expr, env: EnvRef) -> Result<Flow, String> {
    let mut last = JsValue::Undefined;
    loop {
        match execute(body, env.clone())? {
            Flow::Value(v) => last = v,
            Flow::Break(None) => break,
            Flow::Continue(None) => {}
            other => return Ok(other),
        }
        if !eval_expr(cond, env.clone())?.truthy() {
            break;
        }
    }
    Ok(Flow::Value(last))
}

fn execute_try(
    body: &[Stmt],
    catch: Option<&(String, Vec<Stmt>)>,
    finally: Option<&Vec<Stmt>>,
    env: EnvRef,
) -> Result<Flow, String> {
    let result = match (execute_block(body, Env::new(Some(env.clone()))), catch) {
        (Err(error), Some((name, catch_body))) => {
            let catch_env = Env::new(Some(env.clone()));
            catch_env
                .borrow_mut()
                .define(name, caught_error_object(error));
            execute_block(catch_body, catch_env)
        }
        (result, _) => result,
    };
    if let Some(finally) = finally {
        match execute_block(finally, Env::new(Some(env)))? {
            Flow::Value(_) => {}
            other => return Ok(other),
        }
    }
    result
}

fn caught_error_object(error: String) -> JsValue {
    let mut obj = HashMap::new();
    obj.insert("message".into(), JsValue::String(error.clone()));
    obj.insert("stack".into(), JsValue::String(error_stack_text(&error)));
    JsValue::Object(Rc::new(RefCell::new(obj)))
}

fn error_stack_text(error: &str) -> String {
    match error.find(": ") {
        Some(index) => error[..index].into(),
        None => error.into(),
    }
}

fn execute_for(
    init: Option<&Stmt>,
    condition: Option<&Expr>,
    increment: Option<&Expr>,
    body: &Stmt,
    env: EnvRef,
) -> Result<Flow, String> {
    if let Some(init) = init {
        execute(init, env.clone())?;
    }
    let mut last = JsValue::Undefined;
    while condition
        .map(|c| eval_expr(c, env.clone()))
        .transpose()?
        .unwrap_or(JsValue::Bool(true))
        .truthy()
    {
        match execute(body, env.clone())? {
            Flow::Value(v) => last = v,
            Flow::Break(None) => break,
            Flow::Continue(None) => {}
            other => return Ok(other),
        }
        if let Some(increment) = increment {
            eval_expr(increment, env.clone())?;
        }
    }
    Ok(Flow::Value(last))
}

fn execute_for_in(name: &str, iterable: &Expr, body: &Stmt, env: EnvRef) -> Result<Flow, String> {
    let loop_env = Env::new(Some(env.clone()));
    loop_env.borrow_mut().define(name, JsValue::Undefined);
    let mut last = JsValue::Undefined;
    for key in for_in_keys(eval_expr(iterable, env)?) {
        Env::assign(&loop_env, name, JsValue::String(key))?;
        match execute(body, loop_env.clone())? {
            Flow::Value(v) => last = v,
            Flow::Break(None) => break,
            Flow::Continue(None) => continue,
            other => return Ok(other),
        }
    }
    Ok(Flow::Value(last))
}

fn execute_for_of(name: &str, iterable: &Expr, body: &Stmt, env: EnvRef) -> Result<Flow, String> {
    let loop_env = Env::new(Some(env.clone()));
    loop_env.borrow_mut().define(name, JsValue::Undefined);
    let mut last = JsValue::Undefined;
    for value in for_of_values(eval_expr(iterable, env)?) {
        Env::assign(&loop_env, name, value)?;
        match execute(body, loop_env.clone())? {
            Flow::Value(v) => last = v,
            Flow::Break(None) => break,
            Flow::Continue(None) => continue,
            other => return Ok(other),
        }
    }
    Ok(Flow::Value(last))
}

fn for_in_keys(value: JsValue) -> Vec<String> {
    match value {
        JsValue::Object(obj) => own_object_keys(&obj.borrow()),
        JsValue::Array(items) => {
            let mut keys = (0..items.borrow().len())
                .map(|i| i.to_string())
                .collect::<Vec<_>>();
            keys.extend(array_extra_keys(&items));
            keys
        }
        JsValue::String(text) => (0..text.chars().count()).map(|i| i.to_string()).collect(),
        _ => Vec::new(),
    }
}

fn for_of_values(value: JsValue) -> Vec<JsValue> {
    match value {
        JsValue::Array(items) => items.borrow().clone(),
        JsValue::String(text) => text
            .chars()
            .map(|c| JsValue::String(c.to_string()))
            .collect(),
        JsValue::Object(obj) => object_iteration_values(obj),
        _ => Vec::new(),
    }
}

fn object_iteration_values(obj: Rc<RefCell<HashMap<String, JsValue>>>) -> Vec<JsValue> {
    if let Some(values) = iterator_values(JsValue::Object(obj.clone())) {
        return values;
    }
    let obj = obj.borrow();
    if let Some(JsValue::Number(length)) = obj.get("length") {
        return (0..(*length as usize))
            .filter_map(|index| obj.get(&index.to_string()).cloned())
            .collect();
    }
    Vec::new()
}

fn iterator_values(value: JsValue) -> Option<Vec<JsValue>> {
    if let Ok(iterator) = get_property(&value, "Symbol.iterator") {
        if is_callable(&iterator) {
            return drain_iterator(call_with_this(iterator, value, &[]).ok()?);
        }
    }
    drain_iterator(value)
}

fn drain_iterator(iterator: JsValue) -> Option<Vec<JsValue>> {
    let next = get_property(&iterator, "next").ok()?;
    if !is_callable(&next) {
        return None;
    }
    let mut out = Vec::new();
    for _ in 0..100_000 {
        let step = call_with_this(next.clone(), iterator.clone(), &[]).ok()?;
        if get_property(&step, "done").ok().is_some_and(|v| v.truthy()) {
            return Some(out);
        }
        out.push(get_property(&step, "value").unwrap_or(JsValue::Undefined));
    }
    Some(out)
}

fn eval_expr(expr: &Expr, env: EnvRef) -> Result<JsValue, String> {
    js_budget::tick()?;
    match expr {
        Expr::Literal(v) => Ok(v.clone()),
        Expr::Var(name) => env
            .borrow()
            .get(name)
            .ok_or_else(|| format!("ReferenceError: {} is not defined", name)),
        Expr::This => Ok(env.borrow().get("this").unwrap_or(JsValue::Undefined)),
        Expr::Class(class) => class_value(class, env),
        Expr::Function(name, params, body) => Ok(function_value(name.clone(), params, body, env)),
        Expr::Array(items) => {
            let mut out = Vec::new();
            for item in items {
                if let Expr::Spread(inner) = item {
                    out.extend(spread_values(eval_expr(inner, env.clone())?));
                } else {
                    out.push(eval_expr(item, env.clone())?);
                }
            }
            Ok(JsValue::Array(Rc::new(RefCell::new(out))))
        }
        Expr::Object(props) => {
            let mut m = HashMap::new();
            for (k, e) in props {
                if let Expr::Spread(inner) = e {
                    spread_object_into(&mut m, eval_expr(inner, env.clone())?);
                } else {
                    m.insert(k.clone(), eval_expr(e, env.clone())?);
                }
            }
            Ok(JsValue::Object(Rc::new(RefCell::new(m))))
        }
        Expr::Spread(inner) => eval_expr(inner, env),
        Expr::Await(inner) => js_await::value(eval_expr(inner, env)?),
        Expr::Unary(op, e) => {
            let v = eval_expr(e, env)?;
            match op.as_str() {
                "!" => Ok(JsValue::Bool(!v.truthy())),
                "-" => Ok(JsValue::Number(-v.number())),
                "+" => Ok(JsValue::Number(v.number())),
                "~" => Ok(JsValue::Number(!(v.number() as i32) as f64)),
                _ => unreachable!(),
            }
        }
        Expr::Typeof(e) => Ok(JsValue::String(typeof_expr(e, env))),
        Expr::Delete(target) => delete_target(target, env),
        Expr::Update(target, delta, prefix) => eval_update(target, *delta, *prefix, env),
        Expr::Binary(a, op, b) => eval_binary(eval_expr(a, env.clone())?, op, || eval_expr(b, env)),
        Expr::Conditional(c, t, e) => {
            if eval_expr(c, env.clone())?.truthy() {
                eval_expr(t, env)
            } else {
                eval_expr(e, env)
            }
        }
        Expr::Sequence(items) => {
            let mut last = JsValue::Undefined;
            for item in items {
                last = eval_expr(item, env.clone())?;
            }
            Ok(last)
        }
        Expr::Assign(target, rhs) => {
            let v = eval_expr(rhs, env.clone())?;
            assign_target(target, v.clone(), env)?;
            Ok(v)
        }
        Expr::AssignOp(target, op, rhs) => {
            let old = eval_expr(target, env.clone())?;
            let rhs = eval_expr(rhs, env.clone())?;
            let value = eval_binary(old, op, || Ok(rhs))?;
            assign_target(target, value.clone(), env)?;
            Ok(value)
        }
        Expr::Call(callee, args) => {
            let callee_value = eval_callee(callee, env.clone())?;
            let args = eval_args(args, env)?;
            call_value(callee_value, &args)
                .map_err(|error| format!("{} while calling {:?}", error, callee))
        }
        Expr::New(callee, args) => {
            let callee = eval_expr(callee, env.clone())?;
            let args = eval_args(args, env)?;
            construct_value(callee, &args)
        }
        Expr::Get(obj, prop) => get_property(&eval_expr(obj, env)?, prop),
        Expr::OptionalGet(obj, prop) => optional_property(eval_expr(obj, env)?, prop),
        Expr::Index(obj, idx) => {
            let key = eval_expr(idx, env.clone())?.display();
            get_property(&eval_expr(obj, env)?, &key)
        }
        Expr::OptionalIndex(obj, idx) => {
            let base = eval_expr(obj, env.clone())?;
            if nullish(&base) {
                Ok(JsValue::Undefined)
            } else {
                get_property(&base, &eval_expr(idx, env)?.display())
            }
        }
        Expr::OptionalCall(callee, args) => {
            let callee = eval_expr(callee, env.clone())?;
            if nullish(&callee) {
                Ok(JsValue::Undefined)
            } else {
                call_value(callee, &eval_args(args, env)?)
            }
        }
    }
}

fn nullish(value: &JsValue) -> bool {
    matches!(value, JsValue::Undefined | JsValue::Null)
}

fn optional_property(value: JsValue, prop: &str) -> Result<JsValue, String> {
    if nullish(&value) {
        Ok(JsValue::Undefined)
    } else {
        get_property(&value, prop)
    }
}

fn eval_callee(expr: &Expr, env: EnvRef) -> Result<JsValue, String> {
    match expr {
        Expr::Get(obj, prop) => {
            let this_value = eval_expr(obj, env)?;
            Ok(bind_this(get_property(&this_value, prop)?, this_value))
        }
        Expr::Index(obj, idx) => {
            let this_value = eval_expr(obj, env.clone())?;
            let key = eval_expr(idx, env)?.display();
            Ok(bind_this(get_property(&this_value, &key)?, this_value))
        }
        _ => eval_expr(expr, env),
    }
}

fn bind_this(function: JsValue, this_value: JsValue) -> JsValue {
    match function {
        JsValue::Function(_) | JsValue::Native(_) | JsValue::BoundFunction(_) => {
            JsValue::BoundFunction(Rc::new(BoundFunction {
                function,
                this_value,
            }))
        }
        other => other,
    }
}

fn eval_args(args: &[Expr], env: EnvRef) -> Result<Vec<JsValue>, String> {
    let mut out = Vec::new();
    for arg in args {
        if let Expr::Spread(inner) = arg {
            out.extend(spread_values(eval_expr(inner, env.clone())?));
        } else {
            out.push(eval_expr(arg, env.clone())?);
        }
    }
    Ok(out)
}

fn class_value(class: &ClassExpr, env: EnvRef) -> Result<JsValue, String> {
    let superclass = class
        .superclass
        .as_ref()
        .map(|expr| eval_expr(expr, env.clone()))
        .transpose()?;
    let methods = class
        .methods
        .iter()
        .filter(|m| !m.is_static && m.name != "constructor")
        .map(class_method)
        .collect();
    let static_methods = class
        .methods
        .iter()
        .filter(|m| m.is_static)
        .map(class_method)
        .collect();
    let constructor = class
        .methods
        .iter()
        .find(|m| !m.is_static && m.name == "constructor")
        .map(class_method);
    let class = Rc::new(JsClass {
        name: class.name.clone(),
        superclass,
        constructor,
        methods,
        static_methods,
        env,
        properties: RefCell::new(HashMap::new()),
    });
    let prototype = Rc::new(RefCell::new(HashMap::new()));
    install_class_methods(&prototype, &class);
    class
        .properties
        .borrow_mut()
        .insert("prototype".into(), JsValue::Object(prototype));
    Ok(JsValue::Class(class))
}

fn class_method(method: &ClassMethodExpr) -> ClassMethod {
    ClassMethod {
        name: method.name.clone(),
        params: method.params.clone(),
        body: method.body.clone(),
    }
}

fn spread_values(value: JsValue) -> Vec<JsValue> {
    match value {
        JsValue::Array(items) => items.borrow().clone(),
        JsValue::String(text) => text
            .chars()
            .map(|c| JsValue::String(c.to_string()))
            .collect(),
        JsValue::Object(obj) => object_iteration_values(obj),
        _ => Vec::new(),
    }
}

fn spread_object_into(out: &mut HashMap<String, JsValue>, value: JsValue) {
    if let JsValue::Object(obj) = value {
        out.extend(
            obj.borrow()
                .iter()
                .map(|(key, value)| (key.clone(), value.clone())),
        );
    }
}

fn typeof_expr(expr: &Expr, env: EnvRef) -> String {
    match expr {
        Expr::Var(name) if env.borrow().get(name).is_none() => "undefined".into(),
        _ => match eval_expr(expr, env).unwrap_or(JsValue::Undefined) {
            JsValue::Undefined => "undefined".into(),
            JsValue::Null => "object".into(),
            JsValue::Bool(_) => "boolean".into(),
            JsValue::Number(_) => "number".into(),
            JsValue::String(_) => "string".into(),
            JsValue::Symbol(_) => "symbol".into(),
            JsValue::Function(_)
            | JsValue::BoundFunction(_)
            | JsValue::Class(_)
            | JsValue::Native(_) => "function".into(),
            JsValue::Array(_) | JsValue::Object(_) => "object".into(),
        },
    }
}

fn eval_binary(
    left: JsValue,
    op: &str,
    right_eval: impl FnOnce() -> Result<JsValue, String>,
) -> Result<JsValue, String> {
    if op == "&&" {
        return if left.truthy() {
            right_eval()
        } else {
            Ok(left)
        };
    }
    if op == "||" {
        return if left.truthy() {
            Ok(left)
        } else {
            right_eval()
        };
    }
    if op == "??" {
        return if nullish(&left) {
            right_eval()
        } else {
            Ok(left)
        };
    }
    let right = right_eval()?;
    match op {
        "+" => {
            if matches!(left, JsValue::String(_)) || matches!(right, JsValue::String(_)) {
                Ok(JsValue::String(format!(
                    "{}{}",
                    left.display(),
                    right.display()
                )))
            } else {
                Ok(JsValue::Number(left.number() + right.number()))
            }
        }
        "-" => Ok(JsValue::Number(left.number() - right.number())),
        "*" => Ok(JsValue::Number(left.number() * right.number())),
        "**" => Ok(JsValue::Number(left.number().powf(right.number()))),
        "/" => Ok(JsValue::Number(left.number() / right.number())),
        "%" => Ok(JsValue::Number(left.number() % right.number())),
        "&" => Ok(JsValue::Number(
            ((left.number() as i32) & (right.number() as i32)) as f64,
        )),
        "|" => Ok(JsValue::Number(
            ((left.number() as i32) | (right.number() as i32)) as f64,
        )),
        "^" => Ok(JsValue::Number(
            ((left.number() as i32) ^ (right.number() as i32)) as f64,
        )),
        "<<" => Ok(JsValue::Number(
            ((left.number() as i32) << ((right.number() as u32) & 31)) as f64,
        )),
        ">>" => Ok(JsValue::Number(
            ((left.number() as i32) >> ((right.number() as u32) & 31)) as f64,
        )),
        ">>>" => Ok(JsValue::Number(
            ((left.number() as u32) >> ((right.number() as u32) & 31)) as f64,
        )),
        "==" => Ok(JsValue::Bool(loose_equal(&left, &right))),
        "!=" => Ok(JsValue::Bool(!loose_equal(&left, &right))),
        "===" => Ok(JsValue::Bool(left == right)),
        "!==" => Ok(JsValue::Bool(left != right)),
        "<" | "<=" | ">" | ">=" => Ok(JsValue::Bool(compare_relational(&left, op, &right))),
        "in" => Ok(JsValue::Bool(has_property(&right, &left.display()))),
        "instanceof" => Ok(JsValue::Bool(instance_of(&left, &right))),
        _ => unreachable!(),
    }
}

fn compare_relational(left: &JsValue, op: &str, right: &JsValue) -> bool {
    match (left, right) {
        (JsValue::String(left), JsValue::String(right)) => match op {
            "<" => left < right,
            "<=" => left <= right,
            ">" => left > right,
            ">=" => left >= right,
            _ => false,
        },
        _ => match op {
            "<" => left.number() < right.number(),
            "<=" => left.number() <= right.number(),
            ">" => left.number() > right.number(),
            ">=" => left.number() >= right.number(),
            _ => false,
        },
    }
}

fn loose_equal(left: &JsValue, right: &JsValue) -> bool {
    match (left, right) {
        (JsValue::Undefined, JsValue::Null) | (JsValue::Null, JsValue::Undefined) => true,
        (JsValue::Number(_), JsValue::String(_))
        | (JsValue::String(_), JsValue::Number(_))
        | (JsValue::Bool(_), _)
        | (_, JsValue::Bool(_)) => {
            let left = left.number();
            let right = right.number();
            !left.is_nan() && !right.is_nan() && (left - right).abs() < f64::EPSILON
        }
        _ => left == right,
    }
}

fn instance_of(left: &JsValue, right: &JsValue) -> bool {
    if let Some(prototype) = prototype_value(right) {
        return prototype_chain_contains(left, &prototype, 0);
    }
    match right {
        JsValue::Class(_) => matches!(left, JsValue::Object(_)),
        JsValue::Native(native) if native.name == "Object" => matches!(left, JsValue::Object(_)),
        JsValue::Native(native) if native.name == "Array" => matches!(left, JsValue::Array(_)),
        JsValue::Native(native) if native.name == "Uint8Array" => match left {
            JsValue::Array(items) => matches!(
                array_extra_property(items, "__typed_array"),
                Some(JsValue::Bool(true))
            ),
            _ => false,
        },
        JsValue::Native(native)
            if matches!(native.name.as_str(), "ArrayBuffer" | "SharedArrayBuffer") =>
        {
            object_has_marker(left, "__array_buffer")
        }
        _ => false,
    }
}

fn prototype_chain_contains(value: &JsValue, prototype: &JsValue, depth: usize) -> bool {
    if depth > 32 {
        return false;
    }
    let Some(next) = lookup_proto(value) else {
        return false;
    };
    if nullish(&next) || &next == value {
        return false;
    }
    &next == prototype || prototype_chain_contains(&next, prototype, depth + 1)
}

fn object_has_marker(value: &JsValue, marker: &str) -> bool {
    match value {
        JsValue::Object(obj) => obj.borrow().contains_key(marker),
        _ => false,
    }
}

fn own_keys(value: &JsValue) -> Vec<String> {
    match value {
        JsValue::Object(obj) => own_object_keys(&obj.borrow()),
        JsValue::Function(fun) => own_object_keys(&fun.properties.borrow()),
        JsValue::Native(native) => own_object_keys(&native.properties.borrow()),
        JsValue::Class(class) => own_object_keys(&class.properties.borrow()),
        JsValue::Array(items) => {
            let mut keys: Vec<String> = (0..items.borrow().len()).map(|i| i.to_string()).collect();
            keys.extend(array_extra_keys(items));
            keys
        }
        _ => Vec::new(),
    }
}

fn own_object_keys(obj: &HashMap<String, JsValue>) -> Vec<String> {
    let mut out = Vec::new();
    for key in obj.keys() {
        let visible = key
            .strip_prefix("__get:")
            .or_else(|| key.strip_prefix("__set:"))
            .unwrap_or(key);
        if (!is_internal_property(key) || visible != key)
            && !out.iter().any(|existing| existing == visible)
        {
            out.push(visible.to_string());
        }
    }
    out
}

fn is_internal_property(key: &str) -> bool {
    key == "__proto__"
        || key.starts_with("__get:")
        || key.starts_with("__set:")
        || key.starts_with("__regex_")
}

fn has_own_property(value: &JsValue, prop: &str) -> bool {
    match value {
        JsValue::Object(obj) => {
            let obj = obj.borrow();
            obj.contains_key(prop)
                || obj.contains_key(&format!("__get:{prop}"))
                || obj.contains_key(&format!("__set:{prop}"))
        }
        JsValue::Array(items) => {
            prop == "length"
                || prop
                    .parse::<usize>()
                    .is_ok_and(|i| i < items.borrow().len())
                || array_extra_property(items, prop).is_some()
                || accessor_property(value, "get", prop).is_some()
                || accessor_property(value, "set", prop).is_some()
        }
        JsValue::String(text) => {
            prop == "length"
                || prop
                    .parse::<usize>()
                    .is_ok_and(|i| i < text.chars().count())
        }
        JsValue::Function(fun) => has_map_property(&fun.properties.borrow(), prop),
        JsValue::Native(native) => has_map_property(&native.properties.borrow(), prop),
        JsValue::Class(class) => has_map_property(&class.properties.borrow(), prop),
        _ => false,
    }
}

fn has_map_property(map: &HashMap<String, JsValue>, prop: &str) -> bool {
    map.contains_key(prop)
        || map.contains_key(&accessor_key("get", prop))
        || map.contains_key(&accessor_key("set", prop))
}

fn has_property(value: &JsValue, prop: &str) -> bool {
    has_own_property(value, prop) || inherited_has_property(value, prop, 0)
}

fn inherited_has_property(value: &JsValue, prop: &str, depth: usize) -> bool {
    if depth > 32 {
        return false;
    }
    let Some(proto) = lookup_proto(value) else {
        return false;
    };
    !nullish(&proto)
        && (has_own_property(&proto, prop) || inherited_has_property(&proto, prop, depth + 1))
}

fn assign_target(target: &Expr, value: JsValue, env: EnvRef) -> Result<(), String> {
    match target {
        Expr::Var(name) => Env::assign(&env, name, value),
        Expr::Get(obj, prop) => set_property(&eval_expr(obj, env)?, prop, value),
        Expr::Index(obj, idx) => {
            let key = eval_expr(idx, env.clone())?.display();
            set_property(&eval_expr(obj, env)?, &key, value)
        }
        _ => Err("Invalid assignment target".into()),
    }
}

fn delete_target(target: &Expr, env: EnvRef) -> Result<JsValue, String> {
    match target {
        Expr::Get(obj, prop) => remove_property(&eval_expr(obj, env)?, prop),
        Expr::Index(obj, idx) => {
            let key = eval_expr(idx, env.clone())?.display();
            remove_property(&eval_expr(obj, env)?, &key)
        }
        _ => {
            eval_expr(target, env)?;
            Ok(JsValue::Bool(true))
        }
    }
}

fn eval_update(target: &Expr, delta: i32, prefix: bool, env: EnvRef) -> Result<JsValue, String> {
    let old = eval_expr(target, env.clone())?;
    let new = JsValue::Number(old.number() + delta as f64);
    assign_target(target, new.clone(), env)?;
    Ok(if prefix { new } else { old })
}

fn own_property_value(
    owner: &JsValue,
    prop: &str,
    receiver: &JsValue,
) -> Result<Option<JsValue>, String> {
    match owner {
        JsValue::Object(obj) => {
            let getter = obj.borrow().get(&accessor_key("get", prop)).cloned();
            if let Some(getter) = getter {
                return call_with_this(getter, receiver.clone(), &[]).map(Some);
            }
            Ok(obj.borrow().get(prop).cloned())
        }
        JsValue::Array(items) => {
            let getter = array_extra_property(items, &accessor_key("get", prop));
            if let Some(getter) = getter {
                return call_with_this(getter, receiver.clone(), &[]).map(Some);
            }
            Ok(array_extra_property(items, prop))
        }
        JsValue::Function(fun) => own_map_property(&fun.properties.borrow(), prop, receiver),
        JsValue::Native(native) => own_map_property(&native.properties.borrow(), prop, receiver),
        JsValue::Class(class) => own_map_property(&class.properties.borrow(), prop, receiver),
        _ => Ok(None),
    }
}

fn own_map_property(
    map: &HashMap<String, JsValue>,
    prop: &str,
    receiver: &JsValue,
) -> Result<Option<JsValue>, String> {
    if let Some(getter) = map.get(&accessor_key("get", prop)).cloned() {
        return call_with_this(getter, receiver.clone(), &[]).map(Some);
    }
    Ok(map.get(prop).cloned())
}

fn inherited_property(
    owner: &JsValue,
    prop: &str,
    receiver: &JsValue,
    depth: usize,
) -> Result<Option<JsValue>, String> {
    if depth > 32 {
        return Ok(None);
    }
    let Some(proto) = lookup_proto(owner) else {
        return Ok(None);
    };
    if nullish(&proto) {
        return Ok(None);
    }
    if let Some(value) = own_property_value(&proto, prop, receiver)? {
        return Ok(Some(value));
    }
    inherited_property(&proto, prop, receiver, depth + 1)
}

fn get_property(value: &JsValue, prop: &str) -> Result<JsValue, String> {
    match value {
        JsValue::Object(_) => {
            if prop == "inflate" && is_pako_module(value) {
                return Ok(pako_inflate_native());
            }
            if let Some(found) = own_property_value(value, prop, value)? {
                return Ok(found);
            }
            if let Some(found) = inherited_property(value, prop, value, 0)? {
                return Ok(found);
            }
            Ok(object_builtin_method(prop, value.clone()).unwrap_or(JsValue::Undefined))
        }
        JsValue::Array(items) if prop == "length" => {
            Ok(JsValue::Number(items.borrow().len() as f64))
        }
        JsValue::Array(items) => {
            if let Ok(index) = prop.parse::<usize>() {
                return Ok(items
                    .borrow()
                    .get(index)
                    .cloned()
                    .unwrap_or(JsValue::Undefined));
            }
            if let Some(found) = own_property_value(value, prop, value)? {
                return Ok(found);
            }
            let this_value = JsValue::Array(items.clone());
            let found = array_extra_property(items, prop)
                .or_else(|| array_method(prop, items.clone()))
                .or_else(|| js_prototypes::property("Array", prop))
                .or_else(|| object_builtin_method(prop, this_value))
                .unwrap_or(JsValue::Undefined);
            if !matches!(found, JsValue::Undefined) {
                return Ok(found);
            }
            Ok(inherited_property(value, prop, value, 0)?.unwrap_or(JsValue::Undefined))
        }
        JsValue::Bool(_) => {
            Ok(js_prototypes::property("Boolean", prop).unwrap_or(JsValue::Undefined))
        }
        JsValue::Symbol(_) => {
            Ok(js_prototypes::property("Symbol", prop).unwrap_or(JsValue::Undefined))
        }
        JsValue::Number(number) => Ok(number_method(prop, *number)
            .or_else(|| js_prototypes::property("Number", prop))
            .unwrap_or(JsValue::Undefined)),
        JsValue::String(s) if prop == "length" => Ok(JsValue::Number(s.chars().count() as f64)),
        JsValue::String(s) if prop.parse::<usize>().is_ok() => Ok(prop
            .parse::<usize>()
            .ok()
            .and_then(|index| s.chars().nth(index))
            .map(|ch| JsValue::String(ch.to_string()))
            .unwrap_or(JsValue::Undefined)),
        JsValue::String(s) => Ok(string_method(prop, s.clone())
            .or_else(|| js_prototypes::property("String", prop))
            .unwrap_or(JsValue::Undefined)),
        JsValue::Native(native) if native.name == "Promise" => match prop {
            "resolve" => Ok(JsValue::Native(Rc::new(NativeFunction::new(
                "Promise.resolve",
                Some(1),
                promise_resolve,
            )))),
            "reject" => Ok(JsValue::Native(Rc::new(NativeFunction::new(
                "Promise.reject",
                Some(1),
                promise_reject,
            )))),
            _ => Ok(JsValue::Undefined),
        },
        JsValue::Class(class) if prop == "prototype" => Ok(class_prototype_value(class)),
        JsValue::Class(class) => {
            if let Some(found) = own_property_value(value, prop, value)? {
                return Ok(found);
            }
            Ok(class
                .static_methods
                .iter()
                .find(|method| method.name == prop)
                .map(|method| {
                    class_method_function(method, class.env.clone(), class.superclass.clone())
                })
                .unwrap_or(JsValue::Undefined))
        }
        JsValue::Function(fun) => {
            if let Some(method) = function_meta_method(JsValue::Function(fun.clone()), prop) {
                return Ok(method);
            }
            if let Some(found) = own_property_value(value, prop, value)? {
                return Ok(found);
            }
            Ok(inherited_property(value, prop, value, 0)?.unwrap_or(JsValue::Undefined))
        }
        JsValue::Native(native) => {
            if let Some(method) = function_meta_method(JsValue::Native(native.clone()), prop) {
                return Ok(method);
            }
            if let Some(found) = own_property_value(value, prop, value)? {
                return Ok(found);
            }
            Ok(inherited_property(value, prop, value, 0)?.unwrap_or(JsValue::Undefined))
        }
        _ => Ok(JsValue::Undefined),
    }
}

fn is_pako_module(value: &JsValue) -> bool {
    let JsValue::Object(obj) = value else {
        return false;
    };
    let obj = obj.borrow();
    obj.contains_key("inflateRaw") && obj.contains_key("ungzip")
}

fn pako_inflate_native() -> JsValue {
    JsValue::Native(Rc::new(NativeFunction::new("pako.inflate", None, |args| {
        let input = args.first().map(js_byte_values).unwrap_or_default();
        let output = crate::zlib::inflate_zlib(&input)?;
        if pako_wants_string(args.get(1)) {
            return String::from_utf8(output)
                .map(JsValue::String)
                .map_err(|error| format!("pako.inflate: utf8 output failed: {error}"));
        }
        Ok(byte_array_value(output))
    })))
}

fn pako_wants_string(value: Option<&JsValue>) -> bool {
    let Some(JsValue::Object(obj)) = value else {
        return false;
    };
    matches!(obj.borrow().get("to"), Some(JsValue::String(value)) if value == "string")
}

fn js_byte_values(value: &JsValue) -> Vec<u8> {
    match value {
        JsValue::Array(items) => items.borrow().iter().map(js_byte_value).collect(),
        JsValue::Object(obj) => {
            let obj = obj.borrow();
            let len = obj.get("length").map_or(0, |value| value.number() as usize);
            (0..len)
                .map(|index| obj.get(&index.to_string()).map(js_byte_value).unwrap_or(0))
                .collect()
        }
        JsValue::String(text) => text.as_bytes().to_vec(),
        _ => Vec::new(),
    }
}

fn js_byte_value(value: &JsValue) -> u8 {
    (value.number() as i64 & 0xff) as u8
}

fn byte_array_value(bytes: Vec<u8>) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(
        bytes
            .into_iter()
            .map(|byte| JsValue::Number(byte as f64))
            .collect(),
    )))
}

fn get_binding_property(value: &JsValue, path: &str) -> Result<JsValue, String> {
    let mut current = value.clone();
    for prop in path.split('.') {
        current = get_property(&current, prop)?;
    }
    Ok(current)
}

fn get_array_binding_value(source: &[JsValue], path: &str) -> Result<JsValue, String> {
    let mut parts = path.split('.');
    let index = parts.next().unwrap_or("0").parse::<usize>().unwrap_or(0);
    let mut current = source.get(index).cloned().unwrap_or(JsValue::Undefined);
    for prop in parts {
        current = get_property(&current, prop)?;
    }
    Ok(current)
}

fn object_builtin_method(prop: &str, this_value: JsValue) -> Option<JsValue> {
    match prop {
        "hasOwnProperty" => Some(JsValue::Native(Rc::new(NativeFunction::new(
            "Object.bound.hasOwnProperty",
            Some(1),
            move |args| {
                Ok(JsValue::Bool(args.first().is_some_and(|key| {
                    has_own_property(&this_value, &key.display())
                })))
            },
        )))),
        "propertyIsEnumerable" => Some(JsValue::Native(Rc::new(NativeFunction::new(
            "Object.bound.propertyIsEnumerable",
            Some(1),
            move |args| {
                Ok(JsValue::Bool(args.first().is_some_and(|key| {
                    has_own_property(&this_value, &key.display())
                })))
            },
        )))),
        "toString" => Some(JsValue::Native(Rc::new(NativeFunction::new(
            "Object.toString",
            Some(0),
            move |_| Ok(JsValue::String(object_to_string_tag(Some(&this_value)))),
        )))),
        _ => None,
    }
}

fn class_method_function(
    method: &ClassMethod,
    env: EnvRef,
    superclass: Option<JsValue>,
) -> JsValue {
    let mut value = function_value(Some(method.name.clone()), &method.params, &method.body, env);
    if let JsValue::Function(fun) = &mut value {
        let mut copy = (**fun).clone();
        copy.superclass = superclass;
        value = JsValue::Function(Rc::new(copy));
    }
    value
}

fn function_meta_method(function: JsValue, prop: &str) -> Option<JsValue> {
    match prop {
        "call" => Some(JsValue::Native(Rc::new(NativeFunction::new(
            "Function.prototype.call",
            None,
            move |args| function_call(function.clone(), args),
        )))),
        "apply" => Some(JsValue::Native(Rc::new(NativeFunction::new(
            "Function.prototype.apply",
            None,
            move |args| function_apply(function.clone(), args),
        )))),
        "bind" => Some(JsValue::Native(Rc::new(NativeFunction::new(
            "Function.prototype.bind",
            None,
            move |args| function_bind(function.clone(), args),
        )))),
        _ => None,
    }
}

fn function_call(function: JsValue, args: &[JsValue]) -> Result<JsValue, String> {
    let (function, rest) = function_receiver(function, args);
    let this_value = rest.first().cloned().unwrap_or(JsValue::Undefined);
    let rest = rest.get(1..).unwrap_or(&[]);
    call_function_target(function, this_value, rest)
}

fn function_apply(function: JsValue, args: &[JsValue]) -> Result<JsValue, String> {
    let (function, rest) = function_receiver(function, args);
    let this_value = rest.first().cloned().unwrap_or(JsValue::Undefined);
    let applied = rest
        .get(1)
        .map(|value| spread_values(value.clone()))
        .unwrap_or_default();
    call_function_target(function, this_value, &applied)
}

fn function_bind(function: JsValue, args: &[JsValue]) -> Result<JsValue, String> {
    let (function, rest) = function_receiver(function, args);
    let this_value = rest.first().cloned().unwrap_or(JsValue::Undefined);
    let bound_args = rest.get(1..).unwrap_or(&[]).to_vec();
    Ok(JsValue::Native(Rc::new(NativeFunction::new(
        "bound",
        None,
        move |call_args| {
            let mut args = bound_args.clone();
            args.extend_from_slice(call_args);
            call_with_this(function.clone(), this_value.clone(), &args)
        },
    ))))
}

fn function_receiver(function: JsValue, args: &[JsValue]) -> (JsValue, &[JsValue]) {
    match args.first() {
        Some(value) if is_callable(value) => (value.clone(), args.get(1..).unwrap_or(&[])),
        _ => (function, args),
    }
}

fn call_function_target(
    function: JsValue,
    this_value: JsValue,
    args: &[JsValue],
) -> Result<JsValue, String> {
    if native_call_receives_this_arg(&function) {
        let mut call_args = vec![this_value];
        call_args.extend_from_slice(args);
        call_value(function, &call_args)
    } else {
        call_with_this(function, this_value, args)
    }
}

fn native_call_receives_this_arg(function: &JsValue) -> bool {
    let JsValue::Native(native) = function else {
        return false;
    };
    native.name.starts_with("Object.prototype.")
        || native.name.starts_with("Array.prototype.")
        || native.name.starts_with("String.prototype.")
        || native.name.starts_with("RegExp.prototype.")
        || native.name.starts_with("Number.prototype.")
        || native.name.starts_with("Boolean.prototype.")
        || native.name.starts_with("Symbol.prototype.")
        || native.name.starts_with("Function.prototype.")
        || native.name == "Object.hasOwnProperty"
        || native.name.starts_with("Uint8Array.prototype.")
        || native.name.starts_with("Uint32Array.prototype.")
        || native.name.starts_with("Uint16Array.prototype.")
        || native.name.starts_with("Int32Array.prototype.")
        || native.name.starts_with("Float32Array.prototype.")
        || matches!(native.name.as_str(), "forEach" | "map")
}

fn array_method(prop: &str, array: Rc<RefCell<Vec<JsValue>>>) -> Option<JsValue> {
    let name = prop.to_string();
    let method = match prop {
        "push" => NativeFunction::new(name, None, move |args| array_push(array.clone(), args)),
        "pop" => NativeFunction::new(name, Some(0), move |_| array_pop(array.clone())),
        "slice" => NativeFunction::new(name, None, move |args| array_slice(array.clone(), args)),
        "join" => NativeFunction::new(name, None, move |args| array_join(array.clone(), args)),
        "concat" => NativeFunction::new(name, None, move |args| array_concat(array.clone(), args)),
        "forEach" => {
            NativeFunction::new(name, None, move |args| array_for_each(array.clone(), args))
        }
        "map" => NativeFunction::new(name, None, move |args| array_map(array.clone(), args)),
        "filter" => NativeFunction::new(name, None, move |args| array_filter(array.clone(), args)),
        "find" => NativeFunction::new(name, None, move |args| array_find(array.clone(), args)),
        "findIndex" => NativeFunction::new(name, None, move |args| {
            array_find_index(array.clone(), args)
        }),
        "keys" => NativeFunction::new(name, Some(0), move |_| {
            Ok(array_iterator("keys", array.clone()))
        }),
        "values" | "Symbol.iterator" => NativeFunction::new(name, Some(0), move |_| {
            Ok(array_iterator("values", array.clone()))
        }),
        "entries" => NativeFunction::new(name, Some(0), move |_| {
            Ok(array_iterator("entries", array.clone()))
        }),
        "some" => NativeFunction::new(name, None, move |args| array_some(array.clone(), args)),
        "every" => NativeFunction::new(name, None, move |args| array_every(array.clone(), args)),
        "reduce" => NativeFunction::new(name, None, move |args| array_reduce(array.clone(), args)),
        "reduceRight" => NativeFunction::new(name, None, move |args| {
            array_reduce_right(array.clone(), args)
        }),
        "includes" => NativeFunction::new(name, Some(1), move |args| {
            Ok(JsValue::Bool(
                array.borrow().iter().any(|item| item == &args[0]),
            ))
        }),
        "indexOf" => NativeFunction::new(name, Some(1), move |args| {
            Ok(JsValue::Number(
                array
                    .borrow()
                    .iter()
                    .position(|item| item == &args[0])
                    .map(|index| index as f64)
                    .unwrap_or(-1.0),
            ))
        }),
        "sort" => NativeFunction::new(name, None, move |args| array_sort(array.clone(), args)),
        "reverse" => NativeFunction::new(name, Some(0), move |_| array_reverse(array.clone())),
        "shift" => NativeFunction::new(name, Some(0), move |_| array_shift(array.clone())),
        "unshift" => {
            NativeFunction::new(name, None, move |args| array_unshift(array.clone(), args))
        }
        "splice" => NativeFunction::new(name, None, move |args| array_splice(array.clone(), args)),
        _ => return None,
    };
    Some(JsValue::Native(Rc::new(method)))
}

fn array_iterator(kind: &'static str, array: Rc<RefCell<Vec<JsValue>>>) -> JsValue {
    let values = array.borrow().clone();
    let items = values
        .into_iter()
        .enumerate()
        .map(|(index, value)| match kind {
            "keys" => JsValue::Number(index as f64),
            "entries" => JsValue::Array(Rc::new(RefCell::new(vec![
                JsValue::Number(index as f64),
                value,
            ]))),
            _ => value,
        })
        .collect();
    iterator_object(items)
}

fn iterator_object(items: Vec<JsValue>) -> JsValue {
    let index = Rc::new(RefCell::new(0usize));
    let items = Rc::new(items);
    let object = Rc::new(RefCell::new(HashMap::new()));
    let iterator_value = JsValue::Object(object.clone());
    let next_index = index.clone();
    let next_items = items.clone();
    object.borrow_mut().insert(
        "next".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Iterator.next",
            Some(0),
            move |_| Ok(iterator_step(next_index.clone(), next_items.clone())),
        ))),
    );
    let self_value = iterator_value.clone();
    object.borrow_mut().insert(
        "Symbol.iterator".into(),
        JsValue::Native(Rc::new(NativeFunction::new(
            "Iterator.Symbol.iterator",
            Some(0),
            move |_| Ok(self_value.clone()),
        ))),
    );
    iterator_value
}

fn iterator_step(index: Rc<RefCell<usize>>, items: Rc<Vec<JsValue>>) -> JsValue {
    let mut current = index.borrow_mut();
    let done = *current >= items.len();
    let value = if done {
        JsValue::Undefined
    } else {
        let value = items[*current].clone();
        *current += 1;
        value
    };
    let mut step = HashMap::new();
    step.insert("value".into(), value);
    step.insert("done".into(), JsValue::Bool(done));
    JsValue::Object(Rc::new(RefCell::new(step)))
}

fn array_extra_property(array: &Rc<RefCell<Vec<JsValue>>>, prop: &str) -> Option<JsValue> {
    ARRAY_PROPS.with(|props| {
        props
            .borrow()
            .get(&array_id(array))
            .and_then(|values| values.get(prop).cloned())
    })
}

fn set_array_extra_property(array: &Rc<RefCell<Vec<JsValue>>>, prop: &str, value: JsValue) {
    ARRAY_PROPS.with(|props| {
        props
            .borrow_mut()
            .entry(array_id(array))
            .or_default()
            .insert(prop.into(), value);
    });
}

fn remove_array_extra_property(array: &Rc<RefCell<Vec<JsValue>>>, prop: &str) {
    ARRAY_PROPS.with(|props| {
        if let Some(values) = props.borrow_mut().get_mut(&array_id(array)) {
            values.remove(prop);
        }
    });
}

fn array_extra_keys(array: &Rc<RefCell<Vec<JsValue>>>) -> Vec<String> {
    ARRAY_PROPS.with(|props| {
        props
            .borrow()
            .get(&array_id(array))
            .map(own_object_keys)
            .unwrap_or_default()
    })
}

fn array_id(array: &Rc<RefCell<Vec<JsValue>>>) -> usize {
    Rc::as_ptr(array) as usize
}

fn number_method(prop: &str, number: f64) -> Option<JsValue> {
    let method = match prop {
        "toString" => NativeFunction::new("Number.toString", None, move |args| {
            Ok(JsValue::String(number_to_string(
                number,
                args.first().map(|value| value.number() as u32),
            )))
        }),
        "valueOf" => NativeFunction::new("Number.valueOf", Some(0), move |_| {
            Ok(JsValue::Number(number))
        }),
        _ => return None,
    };
    Some(JsValue::Native(Rc::new(method)))
}

fn number_to_string(number: f64, radix: Option<u32>) -> String {
    let radix = radix.unwrap_or(10);
    if !(2..=36).contains(&radix) || radix == 10 || number.fract() != 0.0 || !number.is_finite() {
        return JsValue::Number(number).display();
    }
    let mut value = number as i64;
    let negative = value < 0;
    if negative {
        value = -value;
    }
    let mut digits = Vec::new();
    let mut value = value as u64;
    loop {
        let digit = (value % radix as u64) as u8;
        digits.push(char::from(
            b"0123456789abcdefghijklmnopqrstuvwxyz"[digit as usize],
        ));
        value /= radix as u64;
        if value == 0 {
            break;
        }
    }
    if negative {
        digits.push('-');
    }
    digits.iter().rev().collect()
}

fn string_method(prop: &str, text: String) -> Option<JsValue> {
    let method = match prop {
        "replace" => NativeFunction::new("replace", Some(2), move |args| {
            string_replace(&text, &args[0], &args[1])
        }),
        "toString" => NativeFunction::new("toString", Some(0), move |_| {
            Ok(JsValue::String(text.clone()))
        }),
        "concat" => NativeFunction::new("concat", None, move |args| {
            let mut out = text.clone();
            for arg in args {
                out.push_str(&arg.display());
            }
            Ok(JsValue::String(out))
        }),
        "trim" => NativeFunction::new("trim", Some(0), move |_| {
            Ok(JsValue::String(text.trim().into()))
        }),
        "includes" => NativeFunction::new("includes", Some(1), move |args| {
            Ok(JsValue::Bool(text.contains(&args[0].display())))
        }),
        "startsWith" => NativeFunction::new("startsWith", Some(1), move |args| {
            Ok(JsValue::Bool(text.starts_with(&args[0].display())))
        }),
        "endsWith" => NativeFunction::new("endsWith", Some(1), move |args| {
            Ok(JsValue::Bool(text.ends_with(&args[0].display())))
        }),
        "indexOf" => NativeFunction::new("indexOf", Some(1), move |args| {
            Ok(JsValue::Number(
                text.find(&args[0].display())
                    .map(|index| index as f64)
                    .unwrap_or(-1.0),
            ))
        }),
        "lastIndexOf" => NativeFunction::new("lastIndexOf", Some(1), move |args| {
            Ok(JsValue::Number(
                text.rfind(&args[0].display())
                    .map(|index| index as f64)
                    .unwrap_or(-1.0),
            ))
        }),
        "search" => NativeFunction::new("search", Some(1), move |args| {
            Ok(JsValue::Number(string_search(&text, &args[0])))
        }),
        "split" => NativeFunction::new("split", Some(1), move |args| {
            Ok(JsValue::Array(Rc::new(RefCell::new(
                text.split(&args[0].display())
                    .map(|part| JsValue::String(part.into()))
                    .collect(),
            ))))
        }),
        "substr" => NativeFunction::new("substr", None, move |args| {
            let len = text.chars().count();
            let start = args.first().map_or(0, |v| substr_start(v.number(), len));
            let count = args
                .get(1)
                .map_or(len - start, |v| v.number().max(0.0) as usize);
            Ok(JsValue::String(
                text.chars().skip(start).take(count).collect(),
            ))
        }),
        "match" => NativeFunction::new("match", Some(1), move |args| string_match(&text, &args[0])),
        "toLowerCase" => NativeFunction::new("toLowerCase", Some(0), move |_| {
            Ok(JsValue::String(text.to_lowercase()))
        }),
        "toUpperCase" => NativeFunction::new("toUpperCase", Some(0), move |_| {
            Ok(JsValue::String(text.to_uppercase()))
        }),
        "charAt" => NativeFunction::new("charAt", Some(1), move |args| {
            let index = args[0].number() as usize;
            Ok(JsValue::String(
                text.chars().nth(index).unwrap_or_default().to_string(),
            ))
        }),
        "charCodeAt" => NativeFunction::new("charCodeAt", Some(1), move |args| {
            let index = args[0].number() as usize;
            Ok(text
                .chars()
                .nth(index)
                .map(|ch| JsValue::Number(ch as u32 as f64))
                .unwrap_or(JsValue::Number(f64::NAN)))
        }),
        "codePointAt" => NativeFunction::new("codePointAt", Some(1), move |args| {
            let index = args[0].number() as usize;
            Ok(text
                .chars()
                .nth(index)
                .map(|ch| JsValue::Number(ch as u32 as f64))
                .unwrap_or(JsValue::Undefined))
        }),
        "substring" | "slice" => NativeFunction::new(prop, None, move |args| {
            let len = text.chars().count();
            let start = args
                .first()
                .map_or(0, |v| v.number().max(0.0) as usize)
                .min(len);
            let end = args
                .get(1)
                .map_or(len, |v| v.number().max(0.0) as usize)
                .min(len);
            let (start, end) = if start <= end {
                (start, end)
            } else {
                (end, start)
            };
            Ok(JsValue::String(
                text.chars().skip(start).take(end - start).collect(),
            ))
        }),
        "padStart" => NativeFunction::new("padStart", None, move |args| {
            Ok(JsValue::String(pad_string(&text, args, true)))
        }),
        "padEnd" => NativeFunction::new("padEnd", None, move |args| {
            Ok(JsValue::String(pad_string(&text, args, false)))
        }),
        _ => return None,
    };
    Some(JsValue::Native(Rc::new(method)))
}

fn pad_string(text: &str, args: &[JsValue], start: bool) -> String {
    let target = args
        .first()
        .map_or(0, |value| value.number().max(0.0) as usize);
    let length = text.chars().count();
    if target <= length {
        return text.into();
    }
    let pad = args
        .get(1)
        .map(JsValue::display)
        .filter(|value| !value.is_empty())
        .unwrap_or_else(|| " ".into());
    let fill = repeated_pad(&pad, target - length);
    if start {
        format!("{fill}{text}")
    } else {
        format!("{text}{fill}")
    }
}

fn repeated_pad(pad: &str, count: usize) -> String {
    let mut out = String::new();
    while out.chars().count() < count {
        out.push_str(pad);
    }
    out.chars().take(count).collect()
}

fn string_replace(text: &str, needle: &JsValue, replacement: &JsValue) -> Result<JsValue, String> {
    if let Some((pattern, flags)) = regex_parts(needle) {
        return Ok(JsValue::String(regex_replace(
            text,
            &pattern,
            &flags,
            replacement,
        )?));
    }
    let from = needle.display();
    Ok(JsValue::String(text.replacen(
        &from,
        &replacement.display(),
        1,
    )))
}

fn string_search(text: &str, needle: &JsValue) -> f64 {
    let found = if let Some((pattern, _)) = regex_parts(needle) {
        js_regex_runtime::find(text, &pattern).map(|(start, _)| start)
    } else {
        text.find(&needle.display())
    };
    found.map(|index| index as f64).unwrap_or(-1.0)
}

fn string_match(text: &str, needle: &JsValue) -> Result<JsValue, String> {
    if let Some((pattern, flags)) = regex_parts(needle) {
        return regex_exec(&pattern, &flags, text);
    }
    let needle = needle.display();
    let Some(start) = text.find(&needle) else {
        return Ok(JsValue::Null);
    };
    let array = Rc::new(RefCell::new(vec![JsValue::String(needle)]));
    set_array_extra_property(&array, "index", JsValue::Number(start as f64));
    set_array_extra_property(&array, "input", JsValue::String(text.into()));
    Ok(JsValue::Array(array))
}

fn substr_start(raw: f64, len: usize) -> usize {
    if raw.is_nan() {
        return 0;
    }
    let len_i = len as i64;
    let index = raw.trunc() as i64;
    if index < 0 {
        (len_i + index).max(0) as usize
    } else {
        index.min(len_i) as usize
    }
}

fn regex_parts(value: &JsValue) -> Option<(String, String)> {
    let JsValue::Object(obj) = value else {
        return None;
    };
    let obj = obj.borrow();
    let JsValue::String(pattern) = obj.get("__regex_pattern")? else {
        return None;
    };
    let JsValue::String(flags) = obj.get("__regex_flags")? else {
        return None;
    };
    Some((pattern.clone(), flags.clone()))
}

fn regex_replace(
    text: &str,
    pattern: &str,
    flags: &str,
    replacement: &JsValue,
) -> Result<String, String> {
    if pattern == "[-_\\s]([a-z\\d])(\\w*)" {
        return regex_replace_camel_delimiters(text, flags, replacement);
    }
    let mut out = String::new();
    let mut rest = text;
    while let Some((start, end)) = js_regex_runtime::find(rest, pattern) {
        out.push_str(&rest[..start]);
        let matched = &rest[start..end];
        out.push_str(&replace_text_with_captures(matched, &[], replacement)?);
        rest = &rest[end..];
        if !flags.contains('g') {
            break;
        }
    }
    out.push_str(rest);
    Ok(out)
}

fn replace_text_with_captures(
    matched: &str,
    captures: &[String],
    replacement: &JsValue,
) -> Result<String, String> {
    match replacement {
        JsValue::Function(_)
        | JsValue::BoundFunction(_)
        | JsValue::Class(_)
        | JsValue::Native(_) => {
            let mut args = vec![JsValue::String(matched.into())];
            args.extend(captures.iter().cloned().map(JsValue::String));
            Ok(call_value(replacement.clone(), &args)?.display())
        }
        other => Ok(other.display().replace("$&", matched)),
    }
}

fn regex_replace_camel_delimiters(
    text: &str,
    flags: &str,
    replacement: &JsValue,
) -> Result<String, String> {
    let mut out = String::new();
    let mut rest = text;
    while let Some((start, end, first, tail)) = find_camel_delimiter(rest) {
        out.push_str(&rest[..start]);
        out.push_str(&replace_text_with_captures(
            &rest[start..end],
            &[first, tail],
            replacement,
        )?);
        rest = &rest[end..];
        if !flags.contains('g') {
            break;
        }
    }
    out.push_str(rest);
    Ok(out)
}

fn find_camel_delimiter(text: &str) -> Option<(usize, usize, String, String)> {
    let mut iter = text.char_indices().peekable();
    while let Some((start, ch)) = iter.next() {
        if ch != '-' && ch != '_' && !ch.is_whitespace() {
            continue;
        }
        let (first_start, first) = *iter.peek()?;
        if !first.is_ascii_lowercase() && !first.is_ascii_digit() {
            continue;
        }
        iter.next();
        let mut end = first_start + first.len_utf8();
        while let Some(&(next_start, next)) = iter.peek() {
            if !next.is_ascii_alphanumeric() && next != '_' {
                break;
            }
            iter.next();
            end = next_start + next.len_utf8();
        }
        let tail_start = first_start + first.len_utf8();
        return Some((start, end, first.to_string(), text[tail_start..end].into()));
    }
    None
}

fn array_push(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    let mut array = array.borrow_mut();
    array.extend(args.iter().cloned());
    Ok(JsValue::Number(array.len() as f64))
}

fn array_pop(array: Rc<RefCell<Vec<JsValue>>>) -> Result<JsValue, String> {
    Ok(array.borrow_mut().pop().unwrap_or(JsValue::Undefined))
}

fn array_slice(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    array_slice_values(array.borrow().clone(), args)
}

fn array_slice_values(values: Vec<JsValue>, args: &[JsValue]) -> Result<JsValue, String> {
    let len = values.len();
    let start = args
        .first()
        .map(|v| slice_index(v.number(), len, false))
        .unwrap_or(0);
    let end = args
        .get(1)
        .map(|v| slice_index(v.number(), len, true))
        .unwrap_or(len);
    let end = end.max(start);
    Ok(JsValue::Array(Rc::new(RefCell::new(
        values[start..end].to_vec(),
    ))))
}

fn slice_index(raw: f64, len: usize, default_to_len_on_nan: bool) -> usize {
    if raw.is_nan() {
        return if default_to_len_on_nan { len } else { 0 };
    }
    let len_i = len as i64;
    let index = raw.trunc() as i64;
    let normalized = if index < 0 {
        (len_i + index).max(0)
    } else {
        index.min(len_i)
    };
    normalized as usize
}

fn array_join(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    let sep = args
        .first()
        .map(JsValue::display)
        .unwrap_or_else(|| ",".into());
    let joined = array
        .borrow()
        .iter()
        .map(|value| match value {
            JsValue::Undefined | JsValue::Null => String::new(),
            other => other.display(),
        })
        .collect::<Vec<_>>()
        .join(&sep);
    Ok(JsValue::String(joined))
}

fn array_concat(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    let mut out = array.borrow().clone();
    for arg in args {
        match arg {
            JsValue::Array(items) => out.extend(items.borrow().iter().cloned()),
            other => out.push(other.clone()),
        }
    }
    Ok(JsValue::Array(Rc::new(RefCell::new(out))))
}

fn array_for_each(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    let base = JsValue::Array(array);
    let (snapshot, array_value, args) = js_array_like::receiver(base, args);
    let callback = args
        .first()
        .cloned()
        .ok_or_else(|| "forEach: expected callback".to_string())?;
    for (index, item) in snapshot.into_iter().enumerate() {
        call_value(
            callback.clone(),
            &[item, JsValue::Number(index as f64), array_value.clone()],
        )?;
    }
    Ok(JsValue::Undefined)
}

fn array_map(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    let base = JsValue::Array(array);
    let (snapshot, array_value, args) = js_array_like::receiver(base, args);
    let callback = args
        .first()
        .cloned()
        .ok_or_else(|| "map: expected callback".to_string())?;
    let mut mapped = Vec::with_capacity(snapshot.len());
    for (index, item) in snapshot.into_iter().enumerate() {
        mapped.push(call_value(
            callback.clone(),
            &[item, JsValue::Number(index as f64), array_value.clone()],
        )?);
    }
    Ok(JsValue::Array(Rc::new(RefCell::new(mapped))))
}

fn array_filter(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    let base = JsValue::Array(array);
    let (snapshot, array_value, args) = js_array_like::receiver(base, args);
    let callback = args
        .first()
        .cloned()
        .ok_or_else(|| "filter: expected callback".to_string())?;
    let mut out = Vec::new();
    for (index, item) in snapshot.into_iter().enumerate() {
        if call_value(
            callback.clone(),
            &[
                item.clone(),
                JsValue::Number(index as f64),
                array_value.clone(),
            ],
        )?
        .truthy()
        {
            out.push(item);
        }
    }
    Ok(JsValue::Array(Rc::new(RefCell::new(out))))
}

fn array_find(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    let base = JsValue::Array(array);
    let (snapshot, array_value, args) = js_array_like::receiver(base, args);
    let callback = args
        .first()
        .cloned()
        .ok_or_else(|| "find: expected callback".to_string())?;
    for (index, item) in snapshot.into_iter().enumerate() {
        if call_value(
            callback.clone(),
            &[
                item.clone(),
                JsValue::Number(index as f64),
                array_value.clone(),
            ],
        )?
        .truthy()
        {
            return Ok(item);
        }
    }
    Ok(JsValue::Undefined)
}

fn array_find_index(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    let base = JsValue::Array(array);
    let (snapshot, array_value, args) = js_array_like::receiver(base, args);
    let callback = args
        .first()
        .cloned()
        .ok_or_else(|| "findIndex: expected callback".to_string())?;
    for (index, item) in snapshot.into_iter().enumerate() {
        if call_value(
            callback.clone(),
            &[item, JsValue::Number(index as f64), array_value.clone()],
        )?
        .truthy()
        {
            return Ok(JsValue::Number(index as f64));
        }
    }
    Ok(JsValue::Number(-1.0))
}

fn array_some(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    let base = JsValue::Array(array);
    let (snapshot, array_value, args) = js_array_like::receiver(base, args);
    let callback = args
        .first()
        .cloned()
        .ok_or_else(|| "some: expected callback".to_string())?;
    for (index, item) in snapshot.into_iter().enumerate() {
        if call_value(
            callback.clone(),
            &[item, JsValue::Number(index as f64), array_value.clone()],
        )?
        .truthy()
        {
            return Ok(JsValue::Bool(true));
        }
    }
    Ok(JsValue::Bool(false))
}

fn array_every(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    let base = JsValue::Array(array);
    let (snapshot, array_value, args) = js_array_like::receiver(base, args);
    let callback = args
        .first()
        .cloned()
        .ok_or_else(|| "every: expected callback".to_string())?;
    for (index, item) in snapshot.into_iter().enumerate() {
        if !call_value(
            callback.clone(),
            &[item, JsValue::Number(index as f64), array_value.clone()],
        )?
        .truthy()
        {
            return Ok(JsValue::Bool(false));
        }
    }
    Ok(JsValue::Bool(true))
}

fn array_reduce(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    let base = JsValue::Array(array);
    let (snapshot, array_value, args) = js_array_like::receiver(base, args);
    let callback = args
        .first()
        .cloned()
        .ok_or_else(|| "reduce: expected callback".to_string())?;
    let mut iter = snapshot.into_iter().enumerate();
    let mut acc = if let Some(initial) = args.get(1) {
        initial.clone()
    } else {
        iter.next()
            .map(|(_, value)| value)
            .ok_or_else(|| "reduce: empty array with no initial value".to_string())?
    };
    for (index, item) in iter {
        acc = call_value(
            callback.clone(),
            &[
                acc,
                item,
                JsValue::Number(index as f64),
                array_value.clone(),
            ],
        )?;
    }
    Ok(acc)
}

fn array_reduce_right(
    array: Rc<RefCell<Vec<JsValue>>>,
    args: &[JsValue],
) -> Result<JsValue, String> {
    let base = JsValue::Array(array);
    let (snapshot, array_value, args) = js_array_like::receiver(base, args);
    let callback = args
        .first()
        .cloned()
        .ok_or_else(|| "reduceRight: expected callback".to_string())?;
    let mut index = snapshot.len();
    let mut acc = if let Some(initial) = args.get(1) {
        initial.clone()
    } else {
        index = index
            .checked_sub(1)
            .ok_or_else(|| "reduceRight: empty array with no initial value".to_string())?;
        snapshot[index].clone()
    };
    while index > 0 {
        index -= 1;
        acc = call_value(
            callback.clone(),
            &[
                acc,
                snapshot[index].clone(),
                JsValue::Number(index as f64),
                array_value.clone(),
            ],
        )?;
    }
    Ok(acc)
}

fn array_sort(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    let comparator = args.first().cloned().filter(is_callable);
    let mut items = array.borrow().clone();
    let len = items.len();
    for i in 0..len {
        for j in (i + 1)..len {
            if compare_array_items(&items[i], &items[j], comparator.as_ref())? > 0.0 {
                items.swap(i, j);
            }
        }
    }
    *array.borrow_mut() = items;
    Ok(JsValue::Array(array))
}

fn array_reverse(array: Rc<RefCell<Vec<JsValue>>>) -> Result<JsValue, String> {
    array.borrow_mut().reverse();
    Ok(JsValue::Array(array))
}

fn array_shift(array: Rc<RefCell<Vec<JsValue>>>) -> Result<JsValue, String> {
    if array.borrow().is_empty() {
        return Ok(JsValue::Undefined);
    }
    Ok(array.borrow_mut().remove(0))
}

fn array_unshift(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    let mut items = array.borrow_mut();
    for item in args.iter().rev() {
        items.insert(0, item.clone());
    }
    Ok(JsValue::Number(items.len() as f64))
}

fn array_splice(array: Rc<RefCell<Vec<JsValue>>>, args: &[JsValue]) -> Result<JsValue, String> {
    let mut items = array.borrow_mut();
    let len = items.len();
    let start = args
        .first()
        .map(|value| slice_index(value.number(), len, false))
        .unwrap_or(0);
    let delete_count = args
        .get(1)
        .map(|value| value.number().max(0.0) as usize)
        .unwrap_or(len.saturating_sub(start))
        .min(len.saturating_sub(start));
    let removed = items
        .splice(
            start..start + delete_count,
            args.get(2..).unwrap_or(&[]).to_vec(),
        )
        .collect();
    Ok(JsValue::Array(Rc::new(RefCell::new(removed))))
}

fn compare_array_items(
    left: &JsValue,
    right: &JsValue,
    comparator: Option<&JsValue>,
) -> Result<f64, String> {
    if let Some(comparator) = comparator {
        return Ok(call_value(comparator.clone(), &[left.clone(), right.clone()])?.number());
    }
    Ok(if left.display() > right.display() {
        1.0
    } else if left.display() < right.display() {
        -1.0
    } else {
        0.0
    })
}

fn is_callable(value: &JsValue) -> bool {
    matches!(
        value,
        JsValue::Function(_) | JsValue::BoundFunction(_) | JsValue::Native(_)
    )
}

fn set_property(value: &JsValue, prop: &str, new_value: JsValue) -> Result<(), String> {
    match value {
        JsValue::Object(obj) => {
            let setter_key = format!("__set:{}", prop);
            let setter = obj.borrow().get(&setter_key).cloned();
            let mut stored_value = new_value.clone();
            if let Some(setter) = setter {
                let setter_result =
                    call_with_this(setter, value.clone(), std::slice::from_ref(&new_value))?;
                if !matches!(setter_result, JsValue::Undefined) {
                    stored_value = setter_result;
                }
            }
            obj.borrow_mut().insert(prop.into(), stored_value);
            Ok(())
        }
        JsValue::Array(items) => {
            if prop == "length" {
                items
                    .borrow_mut()
                    .resize(new_value.number().max(0.0) as usize, JsValue::Undefined);
                return Ok(());
            }
            let Ok(idx) = prop.parse::<usize>() else {
                set_array_extra_property(items, prop, new_value);
                return Ok(());
            };
            let mut items = items.borrow_mut();
            if idx >= items.len() {
                items.resize(idx + 1, JsValue::Undefined);
            }
            items[idx] = new_value;
            Ok(())
        }
        JsValue::Function(fun) => {
            fun.properties.borrow_mut().insert(prop.into(), new_value);
            Ok(())
        }
        JsValue::Native(native) => {
            native
                .properties
                .borrow_mut()
                .insert(prop.into(), new_value);
            Ok(())
        }
        JsValue::Class(class) => {
            class.properties.borrow_mut().insert(prop.into(), new_value);
            Ok(())
        }
        _ => Err(format!(
            "Cannot set property {} on {}",
            prop,
            value.display()
        )),
    }
}

pub(crate) fn set_host_property(
    value: &JsValue,
    prop: &str,
    new_value: JsValue,
) -> Result<(), String> {
    set_property(value, prop, new_value)
}

pub(crate) fn get_host_property(value: &JsValue, prop: &str) -> Result<JsValue, String> {
    get_property(value, prop)
}

fn remove_property(value: &JsValue, prop: &str) -> Result<JsValue, String> {
    match value {
        JsValue::Object(obj) => {
            obj.borrow_mut().remove(prop);
            Ok(JsValue::Bool(true))
        }
        JsValue::Array(items) => {
            if let Ok(index) = prop.parse::<usize>() {
                if let Some(slot) = items.borrow_mut().get_mut(index) {
                    *slot = JsValue::Undefined;
                }
            } else {
                remove_array_extra_property(items, prop);
            }
            Ok(JsValue::Bool(true))
        }
        JsValue::Function(fun) => {
            fun.properties.borrow_mut().remove(prop);
            Ok(JsValue::Bool(true))
        }
        JsValue::Native(native) => {
            native.properties.borrow_mut().remove(prop);
            Ok(JsValue::Bool(true))
        }
        _ => Ok(JsValue::Bool(true)),
    }
}

fn call_value(callee: JsValue, args: &[JsValue]) -> Result<JsValue, String> {
    match callee {
        JsValue::Native(native) => {
            if let Some(arity) = native.arity {
                if args.len() < arity {
                    return Err(format!(
                        "{}: expected at least {} args, got {}",
                        native.name,
                        arity,
                        args.len()
                    ));
                }
            }
            (native.func)(args)
        }
        JsValue::BoundFunction(bound) => {
            call_with_this(bound.function.clone(), bound.this_value.clone(), args)
        }
        JsValue::Function(fun) => {
            let call_env = Env::new(Some(fun.env.clone()));
            if let Some(name) = &fun.name {
                call_env
                    .borrow_mut()
                    .define(name, JsValue::Function(fun.clone()));
            }
            bind_super(&call_env, fun.superclass.clone(), JsValue::Undefined);
            bind_params(&call_env, &fun.params, args);
            call_env
                .borrow_mut()
                .define("arguments", arguments_object(args));
            match execute_block(&fun.body, call_env)? {
                Flow::Return(v) => Ok(v),
                Flow::Value(_) => Ok(JsValue::Undefined),
                Flow::Break(_) | Flow::Continue(_) => Err("break/continue outside loop".into()),
            }
        }
        JsValue::Class(class) => Err(format!(
            "TypeError: class {} cannot be invoked without new",
            class.name.as_deref().unwrap_or("")
        )),
        _ => Err(format!("TypeError: {} is not callable", callee.display())),
    }
}

fn bind_params(env: &EnvRef, params: &[String], args: &[JsValue]) {
    for (i, name) in params.iter().enumerate() {
        if let Some(rest) = name.strip_prefix("...") {
            env.borrow_mut().define(
                rest,
                JsValue::Array(Rc::new(RefCell::new(args.get(i..).unwrap_or(&[]).to_vec()))),
            );
            return;
        }
        env.borrow_mut()
            .define(name, args.get(i).cloned().unwrap_or(JsValue::Undefined));
    }
}

fn bind_super(env: &EnvRef, superclass: Option<JsValue>, this_value: JsValue) {
    if let Some(superclass) = superclass {
        env.borrow_mut()
            .define("super", super_value(superclass, this_value));
    }
}

fn super_value(superclass: JsValue, this_value: JsValue) -> JsValue {
    let ctor_super = superclass.clone();
    let ctor_this = this_value.clone();
    let mut native = NativeFunction::new("super", None, move |args| {
        apply_super_constructor(ctor_super.clone(), ctor_this.clone(), args)
    });
    if let JsValue::Class(class) = &superclass {
        for method in &class.methods {
            native = native.with_property(
                method.name.clone(),
                JsValue::BoundFunction(Rc::new(BoundFunction {
                    function: class_method_function(
                        method,
                        class.env.clone(),
                        class.superclass.clone(),
                    ),
                    this_value: this_value.clone(),
                })),
            );
        }
    }
    JsValue::Native(Rc::new(native))
}

fn apply_super_constructor(
    superclass: JsValue,
    this_value: JsValue,
    args: &[JsValue],
) -> Result<JsValue, String> {
    match superclass {
        JsValue::Class(class) => {
            run_class_constructor(&class, this_value.clone(), args)?;
            Ok(this_value)
        }
        JsValue::Function(_) | JsValue::Native(_) => {
            if let JsValue::Object(object) = &this_value {
                install_prototype_properties(object, &superclass);
            }
            call_with_this(superclass, this_value, args)
        }
        JsValue::BoundFunction(_) => call_with_this(superclass, this_value, args),
        JsValue::Undefined | JsValue::Null => Ok(JsValue::Undefined),
        other => Err(format!(
            "TypeError: {} is not a constructor",
            other.display()
        )),
    }
}

fn install_prototype_properties(
    object: &Rc<RefCell<HashMap<String, JsValue>>>,
    constructor: &JsValue,
) {
    if let Some(prototype) = prototype_value(constructor) {
        object
            .borrow_mut()
            .insert("__proto__".into(), prototype.clone());
    }
}

fn prototype_value(value: &JsValue) -> Option<JsValue> {
    match value {
        JsValue::Function(fun) => fun.properties.borrow().get("prototype").cloned(),
        JsValue::Native(native) => native.property("prototype"),
        JsValue::Class(class) => Some(class_prototype_value(class)),
        _ => None,
    }
}

fn construct_value(callee: JsValue, args: &[JsValue]) -> Result<JsValue, String> {
    match callee {
        JsValue::Native(_) => call_value(callee, args),
        JsValue::BoundFunction(bound) => construct_value(bound.function.clone(), args),
        JsValue::Class(class) => construct_class(class, args),
        JsValue::Function(fun) => {
            let object = Rc::new(RefCell::new(HashMap::new()));
            install_prototype_properties(&object, &JsValue::Function(fun.clone()));
            let this_value = JsValue::Object(object);
            let result = call_with_this(JsValue::Function(fun), this_value.clone(), args)?;
            if matches!(
                result,
                JsValue::Array(_)
                    | JsValue::Object(_)
                    | JsValue::Function(_)
                    | JsValue::BoundFunction(_)
                    | JsValue::Native(_)
            ) {
                Ok(result)
            } else {
                Ok(this_value)
            }
        }
        other => Err(format!(
            "TypeError: {} is not a constructor",
            other.display()
        )),
    }
}

fn construct_class(class: Rc<JsClass>, args: &[JsValue]) -> Result<JsValue, String> {
    let object = Rc::new(RefCell::new(HashMap::new()));
    object
        .borrow_mut()
        .insert("__proto__".into(), class_prototype_value(&class));
    let this_value = JsValue::Object(object);
    run_class_constructor(&class, this_value.clone(), args)?;
    Ok(this_value)
}

fn install_class_methods(object: &Rc<RefCell<HashMap<String, JsValue>>>, class: &Rc<JsClass>) {
    if let Some(JsValue::Class(parent)) = &class.superclass {
        install_class_methods(object, parent);
    } else if let Some(superclass) = &class.superclass {
        install_prototype_properties(object, superclass);
    }
    for method in &class.methods {
        object.borrow_mut().insert(
            method.name.clone(),
            class_method_function(method, class.env.clone(), class.superclass.clone()),
        );
    }
}

fn class_prototype_value(class: &Rc<JsClass>) -> JsValue {
    if let Some(prototype) = class.properties.borrow().get("prototype").cloned() {
        return prototype;
    }
    let object = Rc::new(RefCell::new(HashMap::new()));
    install_class_methods(&object, class);
    let prototype = JsValue::Object(object);
    class
        .properties
        .borrow_mut()
        .insert("prototype".into(), prototype.clone());
    prototype
}

fn run_class_constructor(
    class: &Rc<JsClass>,
    this_value: JsValue,
    args: &[JsValue],
) -> Result<(), String> {
    if let Some(constructor) = &class.constructor {
        let function =
            class_method_function(constructor, class.env.clone(), class.superclass.clone());
        call_with_this(function, this_value, args)?;
    }
    Ok(())
}

pub fn call_function_with_this(
    callee: JsValue,
    this_value: JsValue,
    args: &[JsValue],
) -> Result<JsValue, String> {
    call_with_this(callee, this_value, args)
}

fn call_with_this(
    callee: JsValue,
    this_value: JsValue,
    args: &[JsValue],
) -> Result<JsValue, String> {
    match callee {
        JsValue::Function(fun) => {
            let call_env = Env::new(Some(fun.env.clone()));
            call_env.borrow_mut().define("this", this_value.clone());
            bind_super(&call_env, fun.superclass.clone(), this_value.clone());
            bind_params(&call_env, &fun.params, args);
            call_env
                .borrow_mut()
                .define("arguments", arguments_object(args));
            match execute_block(&fun.body, call_env)? {
                Flow::Return(v) => Ok(v),
                Flow::Value(_) => Ok(JsValue::Undefined),
                Flow::Break(_) | Flow::Continue(_) => Err("break/continue outside loop".into()),
            }
        }
        JsValue::BoundFunction(bound) => {
            call_with_this(bound.function.clone(), bound.this_value.clone(), args)
        }
        JsValue::Native(native)
            if native_call_receives_this_arg(&JsValue::Native(native.clone())) =>
        {
            let mut call_args = vec![this_value];
            call_args.extend_from_slice(args);
            call_value(JsValue::Native(native), &call_args)
        }
        other => call_value(other, args),
    }
}

fn arguments_object(args: &[JsValue]) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(args.to_vec())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arithmetic_variables_and_functions_work() {
        assert_eq!(
            eval("let x = 2 + 3 * 4; x;").unwrap(),
            JsValue::Number(14.0)
        );
        assert_eq!(
            eval("function add(a,b){ return a + b; } add(4, 5);").unwrap(),
            JsValue::Number(9.0)
        );
    }

    #[test]
    fn control_flow_arrays_and_objects_work() {
        let src = "let xs=[1,2]; xs[2]=3; let o={name:'Ada'}; let i=0; let sum=0; while(i<xs.length){ sum=sum+xs[i]; i=i+1; } o.name + ':' + sum;";
        assert_eq!(eval(src).unwrap(), JsValue::String("Ada:6".into()));
    }

    #[test]
    fn console_log_is_captured() {
        let mut engine = JsEngine::new();
        engine.eval("console.log('hello', 42);").unwrap();
        assert_eq!(engine.console_output(), vec!["hello 42".to_string()]);
    }

    #[test]
    fn array_methods_work_as_properties_and_globals() {
        let src = "let xs=[1,2]; let pushed=xs.push(3,4); let popped=xs.pop(); let sliced=xs.slice(1).join('|'); let seen=''; xs.forEach(function(v,i){ seen=seen+i+v; }); let mapped=xs.map(function(v,i){ return v+i; }); pushed + ':' + popped + ':' + xs.join('-') + ':' + sliced + ':' + seen + ':' + mapped.join(',') + ':' + push(xs, 9) + ':' + pop(xs);";
        assert_eq!(
            eval(src).unwrap(),
            JsValue::String("4:4:1-2-3:2|3:011223:1,3,5:4:9".into())
        );
    }

    #[test]
    fn new_invokes_native_constructors_like_calls() {
        let mut engine = JsEngine::new();
        engine.set_global(
            "Thing",
            JsValue::Native(Rc::new(NativeFunction::new("Thing", None, |args| {
                let mut obj = HashMap::new();
                obj.insert(
                    "kind".into(),
                    args.first().cloned().unwrap_or(JsValue::Undefined),
                );
                obj.insert(
                    "label".into(),
                    JsValue::Native(Rc::new(NativeFunction::new("Thing.label", Some(0), |_| {
                        Ok(JsValue::String("method".into()))
                    }))),
                );
                Ok(JsValue::Object(Rc::new(RefCell::new(obj))))
            }))),
        );
        assert_eq!(
            engine
                .eval("let a=Thing('call'); let b=new Thing('ctor'); a.kind + ':' + b.kind + ':' + new Thing('x').label();")
                .unwrap(),
            JsValue::String("call:ctor:method".into())
        );
    }

    #[test]
    fn new_constructs_user_functions_with_this_object() {
        assert_eq!(
            eval("function Person(name){ this.name=name; return 7; } let p=new Person('Ada'); p.name + ':' + typeof p;").unwrap(),
            JsValue::String("Ada:object".into())
        );
        assert_eq!(
            eval("function Box(v){ return {value:v}; } new Box(3).value;").unwrap(),
            JsValue::Number(3.0)
        );
    }

    #[test]
    fn promises_support_constructor_then_catch_and_static_helpers() {
        assert_eq!(
            eval("let seen=''; Promise(function(resolve,reject){ resolve('ok'); }).then(function(v){ seen=v+'!'; }); seen;").unwrap(),
            JsValue::String("ok!".into())
        );
        assert_eq!(
            eval("let a=''; Promise.resolve(2).then(function(v){ a='r'+v; }); let b=''; Promise.reject('bad').catch(function(e){ b=e; }); a + ':' + b;").unwrap(),
            JsValue::String("r2:bad".into())
        );
    }
}
