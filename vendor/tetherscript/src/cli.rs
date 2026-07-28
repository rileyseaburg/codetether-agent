//! CLI entry point for the `tetherscript` binary. Lives inside the library so the
//! binary reduces to a thin shim; harnesses embedding TetherScript as a library
//! can ignore this module entirely and drive the core types directly.

use std::env;
use std::fs;
use std::process;
use std::rc::Rc;

use crate::codetether;
use crate::compiler::Compiler;
use crate::experiment;
use crate::fs_cap;
use crate::http_cap;
use crate::interp::Interpreter;
use crate::json;
use crate::lexer::Lexer;
use crate::lsp;
use crate::parser::Parser;
use crate::plugin::{TetherScriptAuthority, PluginError, PluginHost};
use crate::value::{ResultValue, Value};
use crate::vm::VM;

/// Parse argv and execute. Exits the process on failure — intended to be
/// called from `fn main()`.
pub fn run() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("usage: tetherscript [--tokens|--ast|--bytecode|--vm|--lsp] [--step-budget <n>] [--grant-fs <root>] [--grant-http <origin>...] <file.tether>");
        eprintln!("       tetherscript --plugin <plugin.tether> <hook> [json-arg...]");
        eprintln!("       tetherscript --codetether-manifest <plugin.tether>");
        process::exit(2);
    }

    if args[1] == "--lsp" {
        if let Err(e) = lsp::run() {
            eprintln!("tetherscript-lsp: {}", e);
            process::exit(1);
        }
        return;
    }

    if args[1] == "--plugin" {
        process::exit(run_plugin_cli(&args[2..]));
    }
    if args[1] == "--codetether-manifest" {
        process::exit(run_codetether_manifest_cli(&args[2..]));
    }

    // Source-emission experiment harness.
    //   tetherscript --prompt <task_id>                    # dump prompt to stdout
    //   tetherscript --experiment <task_id> <source_file>  # classify a model output
    if args[1] == "--prompt" && args.len() == 3 {
        process::exit(experiment::print_prompt(&args[2]));
    }
    if args[1] == "--experiment" && args.len() == 4 {
        process::exit(experiment::run_from_cli(&args[2], &args[3]));
    }

    let mut mode = "run";
    let mut path: Option<String> = None;
    let mut step_budget: Option<u64> = None;
    let mut fs_grant: Option<String> = None;
    let mut http_grants: Vec<String> = Vec::new();
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--tokens" => {
                mode = "tokens";
                i += 1;
            }
            "--ast" => {
                mode = "ast";
                i += 1;
            }
            "--bytecode" => {
                mode = "bytecode";
                i += 1;
            }
            "--vm" => {
                mode = "vm";
                i += 1;
            }
            "--step-budget" => {
                if i + 1 >= args.len() {
                    eprintln!("tetherscript: --step-budget requires an integer argument");
                    process::exit(2);
                }
                match args[i + 1].parse::<u64>() {
                    Ok(n) => step_budget = Some(n),
                    Err(_) => {
                        eprintln!("tetherscript: --step-budget must be a non-negative integer");
                        process::exit(2);
                    }
                }
                i += 2;
            }
            "--grant-fs" => {
                if i + 1 >= args.len() {
                    eprintln!("tetherscript: --grant-fs requires a directory argument");
                    process::exit(2);
                }
                fs_grant = Some(args[i + 1].clone());
                i += 2;
            }
            "--grant-http" => {
                if i + 1 >= args.len() {
                    eprintln!("tetherscript: --grant-http requires an origin argument");
                    process::exit(2);
                }
                http_grants.push(args[i + 1].clone());
                i += 2;
            }
            other => {
                if path.is_some() {
                    eprintln!("tetherscript: unexpected argument `{}`", other);
                    process::exit(2);
                }
                path = Some(other.to_string());
                i += 1;
            }
        }
    }
    let path = match path {
        Some(p) => p,
        None => {
            eprintln!("tetherscript: missing source file");
            process::exit(2);
        }
    };

    let src = match fs::read_to_string(&path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("tetherscript: can't read {}: {}", path, e);
            process::exit(1);
        }
    };

    let tokens = match Lexer::new(&src).tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("tetherscript: lex error at {}:{}: {}", e.line, e.col, e.msg);
            process::exit(1);
        }
    };

    if mode == "tokens" {
        for t in &tokens {
            println!("{:>3}:{:<3}  {:?}", t.line, t.col, t.token);
        }
        return;
    }

    let program = match Parser::new(tokens).parse_program() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("tetherscript: parse error at {}:{}: {}", e.line, e.col, e.msg);
            process::exit(1);
        }
    };

    if mode == "ast" {
        println!("{:#?}", program);
        return;
    }

    if mode == "bytecode" {
        let chunk = Compiler::compile_program(&program);
        println!("{:#?}", chunk);
        return;
    }

    if mode == "vm" {
        let chunk = Compiler::compile_program(&program);
        let mut vm = VM::new();
        if let Some(root) = &fs_grant {
            vm.grant("fs", fs_cap::FsAuthority::new(root));
        }
        if !http_grants.is_empty() {
            vm.grant("http", http_cap::HttpAuthority::new(http_grants.clone()));
        }
        let result = if let Some(budget) = step_budget {
            crate::interp::with_step_budget(budget, || vm.run(chunk))
        } else {
            vm.run(chunk)
        };
        if let Err(e) = result {
            eprintln!("tetherscript: {}", e);
            process::exit(1);
        }
        return;
    }

    let mut interp = Interpreter::new();
    if let Some(root) = &fs_grant {
        interp.grant("fs", fs_cap::FsAuthority::new(root));
    }
    if !http_grants.is_empty() {
        interp.grant("http", http_cap::HttpAuthority::new(http_grants.clone()));
    }
    let result = if let Some(budget) = step_budget {
        crate::interp::with_step_budget(budget, || interp.run(&program))
    } else {
        interp.run(&program)
    };
    if let Err(e) = result {
        eprintln!("tetherscript: {}", e);
        process::exit(1);
    }
}

fn run_codetether_manifest_cli(args: &[String]) -> i32 {
    if args.len() != 1 {
        eprintln!("usage: tetherscript --codetether-manifest <plugin.tether>");
        return 2;
    }

    match codetether::manifest_for_file(&args[0]) {
        Ok(manifest) => {
            match serde_json::to_string_pretty(&manifest) {
                Ok(json) => println!("{}", json),
                Err(e) => {
                    eprintln!("tetherscript codetether: encode manifest: {}", e);
                    return 1;
                }
            }
            0
        }
        Err(e) => {
            eprintln!("tetherscript codetether: {}", e);
            1
        }
    }
}

fn run_plugin_cli(args: &[String]) -> i32 {
    if args.len() < 2 {
        eprintln!("usage: tetherscript --plugin <plugin.tether> <hook> [json-arg...]");
        return 2;
    }

    let plugin_path = &args[0];
    let hook = &args[1];
    let mut hook_args = Vec::new();
    for raw in &args[2..] {
        match parse_plugin_arg(raw) {
            Ok(value) => hook_args.push(value),
            Err(e) => {
                eprintln!("tetherscript plugin: {}", e);
                return 2;
            }
        }
    }

    let mut host = PluginHost::new();
    host.grant("tetherscript", TetherScriptAuthority::new());

    let mut plugin = match host.load_file(plugin_path) {
        Ok(plugin) => plugin,
        Err(e) => {
            print_plugin_error(&e);
            return 1;
        }
    };
    if !plugin.load_stdout().is_empty() {
        print!("{}", plugin.load_stdout());
    }

    let call = match plugin.call(hook, &hook_args) {
        Ok(call) => call,
        Err(e) => {
            print_plugin_error(&e);
            return 1;
        }
    };
    if !call.stdout.is_empty() {
        print!("{}", call.stdout);
    }

    print_plugin_value(&call.value);
    plugin_exit_code(&call.value)
}

fn print_plugin_error(error: &PluginError) {
    match error {
        PluginError::Load { stdout, .. } | PluginError::Hook { stdout, .. }
            if !stdout.is_empty() =>
        {
            print!("{}", stdout);
        }
        _ => {}
    }
    eprintln!("tetherscript plugin: {}", error);
}

fn parse_plugin_arg(raw: &str) -> Result<Value, String> {
    if let Some(path) = raw.strip_prefix('@') {
        return fs::read_to_string(path)
            .map(|s| Value::Str(Rc::new(s)))
            .map_err(|e| format!("can't read {}: {}", path, e));
    }

    json::parse(&Value::Str(Rc::new(raw.to_string())))
        .or_else(|_| Ok(Value::Str(Rc::new(raw.to_string()))))
}

fn print_plugin_value(value: &Value) {
    match value {
        Value::Nil => {}
        Value::Result(result) => match result.as_ref() {
            ResultValue::Ok(Value::Nil) => {}
            ResultValue::Ok(value) => println!("{}", value),
            ResultValue::Err(message) => eprintln!("Err({:?})", message),
        },
        value => println!("{}", value),
    }
}

fn plugin_exit_code(value: &Value) -> i32 {
    match value {
        Value::Result(result) if matches!(result.as_ref(), ResultValue::Err(_)) => 1,
        _ => 0,
    }
}