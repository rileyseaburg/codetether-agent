pub fn message(unwind: tetherscript::interp::Unwind) -> String {
    use tetherscript::interp::Unwind;

    match unwind {
        Unwind::Error(s) | Unwind::Panic(s) | Unwind::TryErr(s) => s,
        Unwind::Return(_) => "unexpected return".to_string(),
    }
}
