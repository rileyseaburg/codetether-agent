use anyhow::Result;
use serde_json::Value;
use tetherscript::interp::Interpreter;
use tetherscript::lexer::Lexer;
use tetherscript::output;
use tetherscript::parser::Parser;
use tetherscript::value::{ResultValue, Value as TetherScriptValue};

use super::convert::{json_to_tetherscript, tetherscript_to_json};

const OUTPUT_LIMIT: usize = 64 * 1024;

#[derive(Debug)]
pub struct TetherScriptOutcome {
    pub output: String,
    pub success: bool,
    pub value: Value,
}

pub fn run(
    source_name: String,
    source: String,
    hook: String,
    args: Vec<Value>,
) -> Result<TetherScriptOutcome> {
    let tokens = Lexer::new(&source).tokenize().map_err(|e| {
        anyhow::anyhow!("{}:{}: {}", source_name, e.line, e.msg)
    })?;
    let program = Parser::new(tokens).parse_program().map_err(|e| {
        anyhow::anyhow!("{}:{}: {}", source_name, e.line, e.msg)
    })?;
    let mut interp = Interpreter::new();
    interp.run_repl(&program).map_err(|e| {
        anyhow::anyhow!("{source_name}: load failed: {e}")
    })?;
    let callee = interp
        .globals
        .borrow()
        .get(&hook)
        .map_err(|e| anyhow::anyhow!("{source_name}: hook '{hook}': {e}"))?;
    let args: Vec<_> = args.into_iter().map(json_to_tetherscript).collect();
    let (stdout, call_result) = output::with_capture(OUTPUT_LIMIT, || {
        interp.call(&callee, &args)
    });
    let (tether_val, success) = match call_result {
        Ok(v) => {
            let ok = !matches!(&v, TetherScriptValue::Result(r) if matches!(r.as_ref(), ResultValue::Err(_)));
            (v, ok)
        }
        Err(unwind) => (TetherScriptValue::Str(std::rc::Rc::new(unwind_msg(unwind))), false),
    };
    let value = tetherscript_to_json(&tether_val);
    let output = format_output(stdout, &tether_val);
    Ok(TetherScriptOutcome { output, success, value })
}

fn format_output(mut stdout: String, value: &TetherScriptValue) -> String {
    let empty = matches!(value, TetherScriptValue::Nil)
        || matches!(value, TetherScriptValue::Result(r) if matches!(r.as_ref(), ResultValue::Ok(TetherScriptValue::Nil)));
    if !empty {
        if !stdout.is_empty() && !stdout.ends_with('\n') { stdout.push('\n'); }
        stdout.push_str(&value.to_string());
    }
    stdout
}

fn unwind_msg(unwind: tetherscript::interp::Unwind) -> String {
    use tetherscript::interp::Unwind;
    match unwind {
        Unwind::Error(s) | Unwind::Panic(s) | Unwind::TryErr(s) => s,
        Unwind::Return(_) => "unexpected return".to_string(),
    }
}
