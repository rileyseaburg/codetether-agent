//! Source-emission experiment harness.
//!
//! Given TetherScript source produced by a model and a task spec, classifies the
//! outcome as one of:
//!
//!   SUCCESS    — parses, executes cleanly, task success-check passes.
//!   TASK       — parses, executes cleanly, but success-check fails.
//!                (Program ran, but didn't do what was asked.)
//!   SEMANTIC   — parses, but runtime fails.
//!                (Wrong capability, wrong method, wrong args, path escape,
//!                 unhandled ?-error, use-after-move, revocation, etc.)
//!   SYNTACTIC  — lex or parse error.
//!
//! These categories have distinct implications for what to build next:
//!   - SYNTACTIC dominant → parser/prompt work, or move to bytecode emission
//!   - SEMANTIC dominant → capability surface rough edges, or training data
//!   - TASK dominant → model reasoning gap, not a language problem
//!   - SUCCESS dominant → the bet works; scale up

use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

use crate::capability::Authority;
use crate::fs_cap::FsAuthority;
use crate::http_cap::HttpAuthority;
use crate::interp::Interpreter;
use crate::lexer::Lexer;
use crate::output;
use crate::parser::Parser;

pub struct TaskSpec {
    pub id: &'static str,
    pub description: &'static str,
    pub prompt: &'static str,
    /// Origins the agent is granted HTTP to, if any.
    pub http_origins: Vec<String>,
    /// If Some, grant `fs` rooted at a fresh subdir under `base_workspace`.
    pub fs_scope: bool,
    /// Called after a clean run to judge task success.
    pub success_check: fn(workspace: &Path) -> Result<(), String>,
}

pub struct TrajectoryResult {
    pub outcome: Outcome,
    pub stdout: String,
    pub workspace: PathBuf,
}

pub enum Outcome {
    Success,
    Task(String),
    Semantic(String),
    Syntactic(String),
}

impl Outcome {
    pub fn category(&self) -> &'static str {
        match self {
            Outcome::Success => "SUCCESS",
            Outcome::Task(_) => "TASK",
            Outcome::Semantic(_) => "SEMANTIC",
            Outcome::Syntactic(_) => "SYNTACTIC",
        }
    }
    pub fn detail(&self) -> &str {
        match self {
            Outcome::Success => "",
            Outcome::Task(s) | Outcome::Semantic(s) | Outcome::Syntactic(s) => s,
        }
    }
}

/// All currently-defined tasks. Add rows here to expand the suite.
pub fn tasks() -> Vec<TaskSpec> {
    vec![TaskSpec {
        id: "fetch-and-save",
        description: "Fetch https://example.com/ and save its body to \
                example.html inside the granted fs scope. Program must use \
                Result + `?` for error handling and print a short summary.",
        prompt: FETCH_AND_SAVE_PROMPT,
        http_origins: vec!["https://example.com".into()],
        fs_scope: true,
        success_check: check_fetch_and_save,
    }]
}

/// Run one trajectory: lex → parse → execute with granted caps → check.
/// The workspace is a fresh temp directory; `fs` is rooted there when granted.
/// Program stdout is captured rather than forwarded; it lives inside the
/// returned struct so the distillation pipeline can see it.
pub fn run_trajectory(spec: &TaskSpec, source: &str) -> TrajectoryResult {
    let workspace = fresh_workspace(spec.id);
    let _ = fs::create_dir_all(&workspace);

    // 1. Lex
    let tokens = match Lexer::new(source).tokenize() {
        Ok(t) => t,
        Err(e) => {
            return TrajectoryResult {
                outcome: Outcome::Syntactic(format!(
                    "lex error at {}:{}: {}",
                    e.line, e.col, e.msg
                )),
                stdout: String::new(),
                workspace,
            }
        }
    };

    // 2. Parse
    let program = match Parser::new(tokens).parse_program() {
        Ok(p) => p,
        Err(e) => {
            return TrajectoryResult {
                outcome: Outcome::Syntactic(format!(
                    "parse error at {}:{}: {}",
                    e.line, e.col, e.msg
                )),
                stdout: String::new(),
                workspace,
            }
        }
    };

    // 3. Build an interpreter, grant capabilities, run with captured stdout.
    let mut interp = Interpreter::new();
    if spec.fs_scope {
        interp.grant("fs", FsAuthority::new(&workspace));
    }
    if !spec.http_origins.is_empty() {
        let auth: Rc<dyn Authority> = HttpAuthority::new(spec.http_origins.clone());
        interp.grant("http", auth);
    }

    let (captured, run_result) = output::with_capture(1024 * 1024, || interp.run(&program));

    if let Err(e) = run_result {
        return TrajectoryResult {
            outcome: Outcome::Semantic(e),
            stdout: captured,
            workspace,
        };
    }

    // 4. Task-level check.
    let outcome = match (spec.success_check)(&workspace) {
        Ok(()) => Outcome::Success,
        Err(e) => Outcome::Task(e),
    };
    TrajectoryResult {
        outcome,
        stdout: captured,
        workspace,
    }
}

/// CLI entry point. Wired from main.rs as `tetherscript --experiment <task_id> <source_file>`.
/// Prints a JSON record to stderr (so stdout can be redirected separately
/// for program output capture). Returns 0 on SUCCESS, 1 otherwise.
pub fn run_from_cli(task_id: &str, source_path: &str) -> i32 {
    let spec = match tasks().into_iter().find(|t| t.id == task_id) {
        Some(s) => s,
        None => {
            eprintln!(
                "unknown task id `{}`. known: {:?}",
                task_id,
                tasks().iter().map(|t| t.id).collect::<Vec<_>>()
            );
            return 2;
        }
    };
    let source = match fs::read_to_string(source_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("can't read {}: {}", source_path, e);
            return 2;
        }
    };

    let result = run_trajectory(&spec, &source);
    let record = serde_json::json!({
        "task":      spec.id,
        "category":  result.outcome.category(),
        "detail":    result.outcome.detail(),
        "stdout":    result.stdout,
        "workspace": result.workspace.display().to_string(),
        "source":    source,
    });
    eprintln!("{}", record);
    match result.outcome {
        Outcome::Success => 0,
        _ => 1,
    }
}

/// Print the prompt for a task to stdout (so you can pipe it to a model).
pub fn print_prompt(task_id: &str) -> i32 {
    match tasks().into_iter().find(|t| t.id == task_id) {
        Some(s) => {
            print!("{}", s.prompt);
            0
        }
        None => {
            eprintln!("unknown task id `{}`", task_id);
            2
        }
    }
}

fn fresh_workspace(task_id: &str) -> PathBuf {
    let stamp = chrono::Utc::now().format("%Y%m%dT%H%M%S%.3f").to_string();
    let mut p = std::env::temp_dir();
    p.push(format!("tetherscript-exp-{}-{}", task_id, stamp));
    p
}

// ---------- task: fetch-and-save ----------

fn check_fetch_and_save(workspace: &Path) -> Result<(), String> {
    let target = workspace.join("example.html");
    if !target.exists() {
        return Err("expected file `example.html` was not written".to_string());
    }
    let bytes = fs::read(&target).map_err(|e| format!("read check: {}", e))?;
    if bytes.is_empty() {
        return Err("example.html exists but is empty".into());
    }
    // A minimal sanity check — example.com responses contain this.
    let s = String::from_utf8_lossy(&bytes);
    if !s.to_lowercase().contains("<html") {
        return Err("example.html does not look like HTML".into());
    }
    Ok(())
}

const FETCH_AND_SAVE_PROMPT: &str = r#"# Task: fetch-and-save

You are writing a program in TetherScript, a small dynamically-typed language with
Rust-style syntax and capability-based I/O. Write a complete TetherScript program that
accomplishes the task below, and output ONLY the program source (no prose,
no markdown fences, no explanation).

## Task
Fetch `https://example.com/` via HTTP GET. If the response is OK, write the
full response body to a file named `example.html` in the filesystem workspace
you have been granted. Print a one-line summary at the end indicating whether
you succeeded. Use `Result` and `?` for error handling; do not panic.

## The TetherScript surface available to you

### Values
Ints, floats, strings, bools, nil. Lists: `let xs = []; xs.push(1)`.
Maps: `let m = map(); m["key"] = value`. Note: **there is no map-literal syntax**
— always use `map()` + `m["k"] = v`.

Results: `Ok(v)` and `Err("message")` are constructors. `r?` short-circuits to
the enclosing fn's return if `r` is `Err`. `r.unwrap()`, `r.is_ok()`, `r.is_err()`,
`r.unwrap_or(default)` are available.

### Bindings
`let x = ...` (immutable) or `let mut x = ...` (assignable).

### Control flow
`if cond { ... } else { ... }`, `while cond { ... }`, `fn name(params) { ... }`.
Blocks are expressions — the last expression is the value.

### Capabilities (pre-granted globals)
`http` — HTTP capability.
  - `http.get(url)` → `Result<map, str>` where the map has `status` (int),
    `ok` (bool), `headers` (map), `body` (str).
  - `http.post(url, body)`, `http.head(url)` — same return shape.
  - `http.narrow(params)` — returns a narrowed http capability.

`fs` — filesystem capability.
  - `fs.read(path)` → `Result<str, str>`
  - `fs.write(path, content)` → `Result<nil, str>`
  - `fs.list(path)` → `Result<list, str>`
  - `fs.exists(path)` → `Result<bool, str>`
  - `fs.narrow(params)` — returns a narrowed fs capability.
  - All paths are relative to the granted workspace root. `..` escapes
    and absolute paths are rejected at the capability layer.

### Built-ins (ambient)
`println(...)`, `print(...)`, `len(x)`, `type_of(x)`, `map()`, `Ok(v)`,
`Err(msg)`.

### Conventions
Programs typically define a `fn main()` which is invoked after top-level
declarations are hoisted. Capability globals (`http`, `fs`) are already bound.

## Output format
Emit ONLY TetherScript source code, with no leading or trailing prose, no code fences.
"#;
