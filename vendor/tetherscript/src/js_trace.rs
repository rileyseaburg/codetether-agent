//! Optional JavaScript engine phase tracing.

pub(crate) fn mark(phase: &str) {
    if std::env::var_os("TETHERSCRIPT_JS_TRACE").is_some() {
        eprintln!("js: {phase}");
    }
}

pub(crate) fn parser_progress<T: std::fmt::Debug>(pos: usize, len: usize, kind: &T) {
    if pos.is_multiple_of(5000) && std::env::var_os("TETHERSCRIPT_JS_TRACE").is_some() {
        eprintln!("js: parser token {pos}/{len} {kind:?}");
    }
}
