use std::env;
use std::fs;

use tetherscript::fs_cap::FsAuthority;
use tetherscript::interp::Interpreter;
use tetherscript::lexer::Lexer;
use tetherscript::parser::Parser;

fn main() -> Result<(), String> {
    let mut args = env::args().skip(1);
    let root = args.next().unwrap_or_else(|| ".".to_string());
    let source_path = args
        .next()
        .unwrap_or_else(|| "examples/policy.tether".to_string());

    let source =
        fs::read_to_string(&source_path).map_err(|error| format!("read {source_path}: {error}"))?;
    let tokens = Lexer::new(&source)
        .tokenize()
        .map_err(|error| format!("lex {}:{}: {}", error.line, error.col, error.msg))?;
    let program = Parser::new(tokens)
        .parse_program()
        .map_err(|error| format!("parse {}:{}: {}", error.line, error.col, error.msg))?;

    let mut interp = Interpreter::new();
    interp.grant("fs", FsAuthority::new(root));
    interp.run(&program)
}
