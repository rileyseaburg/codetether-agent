use anyhow::Result;
use tetherscript::interp::Interpreter;
use tetherscript::lexer::Lexer;
use tetherscript::parser::Parser;

pub fn interpreter(source_name: &str, source: &str) -> Result<Interpreter> {
    let tokens = Lexer::new(source)
        .tokenize()
        .map_err(|e| anyhow::anyhow!("{}:{}: {}", source_name, e.line, e.msg))?;
    let program = Parser::new(tokens)
        .parse_program()
        .map_err(|e| anyhow::anyhow!("{}:{}: {}", source_name, e.line, e.msg))?;
    let mut interp = Interpreter::new();
    interp
        .run_repl(&program)
        .map_err(|e| anyhow::anyhow!("{source_name}: load failed: {e}"))?;
    Ok(interp)
}
