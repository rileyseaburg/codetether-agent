use tetherscript::lexer::Lexer;
use tetherscript::ownership::{analyze, Diagnostic};
use tetherscript::parser::Parser;

fn analyze_source(source: &str) -> Result<(), Vec<Diagnostic>> {
    let tokens = Lexer::new(source).tokenize().unwrap();
    let program = Parser::new(tokens).parse_program().unwrap();
    analyze(&program)
}

#[test]
fn moving_copy_scalar_keeps_original_binding_live() {
    let source = "fn main() { let n = 42 let m = move n n }";

    analyze_source(source).unwrap();
}

#[test]
fn moving_heap_value_marks_original_binding_moved() {
    let source = "fn main() { let xs = [1] let ys = move xs xs.len() }";

    let err = analyze_source(source).unwrap_err();

    assert!(err
        .iter()
        .any(|diag| diag.message.contains("use of moved value `xs`")));
}
