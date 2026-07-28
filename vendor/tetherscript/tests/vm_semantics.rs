use tetherscript::compiler::Compiler;
use tetherscript::lexer::Lexer;
use tetherscript::parser::Parser;
use tetherscript::vm::VM;

fn run_vm(source: &str) -> Result<(), String> {
    let tokens = Lexer::new(source).tokenize().unwrap();
    let program = Parser::new(tokens).parse_program().unwrap();
    VM::new().run(Compiler::compile_program(&program))
}

#[test]
fn function_params_are_env_bindings() {
    let source = "fn id(x) { return x } fn main() { assert(id(41) + 1 == 42, \"param\") }";

    run_vm(source).unwrap();
}

#[test]
fn moved_function_local_reports_use_after_move() {
    let source = "fn main() { let xs = [1] let ys = move xs ys.len() xs.len() }";

    let err = run_vm(source).unwrap_err();

    assert!(err.contains("use of moved value `xs`"), "{err}");
}

#[test]
fn immutable_function_local_assignment_is_rejected() {
    let source = "fn main() { let x = 1 x = 2 }";

    let err = run_vm(source).unwrap_err();

    assert!(
        err.contains("cannot assign to immutable binding `x`"),
        "{err}"
    );
}
