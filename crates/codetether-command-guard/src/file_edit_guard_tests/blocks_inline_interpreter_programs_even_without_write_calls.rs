//! File-edit guard regression.
use super::*;

/// Agents route around the `.write(` check with `Path.write_text`, `shutil`,
/// or `os.replace`. Any inline program is unreviewable, so the shape itself
/// is blocked regardless of what it does.
#[test]
fn blocks_inline_interpreter_programs_even_without_write_calls() {
    for command in [
        "cd repo && python3 - <<'EOF'\nimport pathlib\npathlib.Path('x').write_text('y')\nEOF",
        "python3 -c 'import shutil; shutil.copy(\"a\",\"b\")'",
        "/usr/bin/python3.12 -c 'print(1)'",
        "perl -e 'print 1'",
        "perl -pi -e 's/a/b/' file.rs",
        "node -e 'require(\"fs\").writeFileSync(\"x\",\"y\")'",
        "ruby -e 'File.write(\"x\",\"y\")'",
    ] {
        assert!(file_edit_guard_reason(command).is_some(), "{command}");
    }
}
