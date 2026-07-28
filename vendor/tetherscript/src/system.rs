//! Std-only host tools for trusted TetherScript scripts.
//!
//! These builtins are intentionally installed only in the normal CLI/runtime,
//! not in the sandboxed `eval()` runtime.

use std::cell::RefCell;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{ChildStdin, Command, ExitStatus, Stdio};
use std::rc::Rc;
use std::thread;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use crate::value::{ResultValue, Value};

const MAX_FILE_BYTES: usize = 1024 * 1024;
const MAX_PROCESS_IO_BYTES: usize = 1024 * 1024;
const DEFAULT_PROCESS_TIMEOUT_MS: u64 = 30_000;
const MAX_PROCESS_TIMEOUT_MS: u64 = 300_000;
const MAX_SLEEP_MS: i64 = 60_000;

struct PipeOutput {
    text: String,
    truncated: bool,
}

pub fn value_to_string(value: &Value) -> Value {
    Value::Str(Rc::new(value.to_string()))
}

pub fn parse_int(value: &Value) -> Value {
    result_value(parse_int_inner(value))
}

pub fn parse_float(value: &Value) -> Value {
    result_value(parse_float_inner(value))
}

pub fn assert_builtin(args: &[Value]) -> Result<Value, String> {
    if !(1..=2).contains(&args.len()) {
        return Err("assert expects condition[, message]".into());
    }
    if args[0].truthy() {
        return Ok(Value::Nil);
    }
    let message = match args.get(1) {
        Some(Value::Str(message)) => (**message).clone(),
        Some(value) => value.to_string(),
        None => "assertion failed".to_string(),
    };
    Err(message)
}

pub fn time_now_ms() -> Value {
    match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(duration) => Value::Int(duration.as_millis().min(i64::MAX as u128) as i64),
        Err(_) => Value::Int(0),
    }
}

pub fn process_args() -> Value {
    string_list_value(env::args())
}

pub fn process_pid() -> Value {
    Value::Int(std::process::id() as i64)
}

pub fn process_platform() -> Value {
    Value::Str(Rc::new(env::consts::OS.into()))
}

pub fn process_arch() -> Value {
    Value::Str(Rc::new(env::consts::ARCH.into()))
}

pub fn os_platform() -> Value {
    process_platform()
}

pub fn os_arch() -> Value {
    process_arch()
}

pub fn os_tmpdir() -> Value {
    Value::Str(Rc::new(env::temp_dir().to_string_lossy().into_owned()))
}

pub fn os_homedir() -> Value {
    home_dir()
        .map(|path| Value::Str(Rc::new(path.to_string_lossy().into_owned())))
        .unwrap_or(Value::Nil)
}

pub fn os_eol() -> Value {
    Value::Str(Rc::new(if cfg!(windows) { "\r\n" } else { "\n" }.into()))
}

pub fn sleep_ms(value: &Value) -> Value {
    result_value(sleep_ms_inner(value))
}

pub fn env_get(value: &Value) -> Value {
    result_value(env_get_inner(value))
}

pub fn cwd() -> Value {
    result_value(
        env::current_dir()
            .map(|path| Value::Str(Rc::new(path.to_string_lossy().into_owned())))
            .map_err(|error| format!("cwd: {}", error)),
    )
}

pub fn chdir(value: &Value) -> Value {
    result_value(chdir_inner(value))
}

pub fn fs_read(value: &Value) -> Value {
    result_value(fs_read_inner(value))
}

pub fn fs_write(path: &Value, body: &Value) -> Value {
    result_value(fs_write_inner(path, body))
}

pub fn fs_exists(value: &Value) -> Value {
    result_value(path_arg(value, "fs_exists: path").map(|path| Value::Bool(path.exists())))
}

pub fn fs_list(value: &Value) -> Value {
    result_value(fs_list_inner(value))
}

pub fn fs_mkdir(value: &Value) -> Value {
    result_value(fs_mkdir_inner(value))
}

pub fn fs_stat(value: &Value) -> Value {
    result_value(fs_stat_inner(value))
}

pub fn fs_remove(value: &Value) -> Value {
    result_value(fs_remove_inner(value))
}

pub fn fs_rename(from: &Value, to: &Value) -> Value {
    result_value(fs_rename_inner(from, to))
}

pub fn fs_copy(from: &Value, to: &Value) -> Value {
    result_value(fs_copy_inner(from, to))
}

pub fn process_run(args: &[Value]) -> Value {
    result_value(process_run_inner(args))
}

pub fn path_sep() -> Value {
    Value::Str(Rc::new(std::path::MAIN_SEPARATOR.to_string()))
}

pub fn path_join(value: &Value) -> Value {
    result_value(path_join_inner(value))
}

pub fn path_dirname(value: &Value) -> Value {
    result_value(path_part(value, "path_dirname: path", |path| {
        path.parent()
            .map(path_to_value)
            .unwrap_or_else(|| Value::Str(Rc::new(".".into())))
    }))
}

pub fn path_basename(value: &Value) -> Value {
    result_value(path_part(value, "path_basename: path", |path| {
        path.file_name()
            .map(|name| Value::Str(Rc::new(name.to_string_lossy().into_owned())))
            .unwrap_or_else(|| Value::Str(Rc::new(String::new())))
    }))
}

pub fn path_extname(value: &Value) -> Value {
    result_value(path_part(value, "path_extname: path", |path| {
        path.extension()
            .map(|ext| Value::Str(Rc::new(format!(".{}", ext.to_string_lossy()))))
            .unwrap_or_else(|| Value::Str(Rc::new(String::new())))
    }))
}

pub fn path_normalize(value: &Value) -> Value {
    result_value(
        path_arg(value, "path_normalize: path").map(|path| path_to_value(&normalize_path(&path))),
    )
}

pub fn path_resolve(value: &Value) -> Value {
    result_value(path_resolve_inner(value))
}

pub fn sha256_hex(value: &Value) -> Value {
    result_value(
        string_arg(value, "sha256_hex: input")
            .map(|input| Value::Str(Rc::new(hex_encode(&sha256(input.as_bytes()))))),
    )
}

pub fn base64_encode(value: &Value) -> Value {
    result_value(
        string_arg(value, "base64_encode: input")
            .map(|input| Value::Str(Rc::new(base64_encode_bytes(input.as_bytes())))),
    )
}

pub fn base64_decode(value: &Value) -> Value {
    result_value(base64_decode_inner(value))
}

pub fn url_parse(value: &Value) -> Value {
    result_value(url_parse_inner(value))
}

pub(crate) fn result_value(result: Result<Value, String>) -> Value {
    Value::Result(Rc::new(match result {
        Ok(value) => ResultValue::Ok(value),
        Err(error) => ResultValue::Err(error),
    }))
}

fn parse_int_inner(value: &Value) -> Result<Value, String> {
    match value {
        Value::Int(value) => Ok(Value::Int(*value)),
        Value::Float(value) if value.fract() == 0.0 => Ok(Value::Int(*value as i64)),
        Value::Str(value) => value
            .trim()
            .parse::<i64>()
            .map(Value::Int)
            .map_err(|error| format!("parse_int: {}", error)),
        other => Err(format!(
            "parse_int: expected str/int, got {}",
            other.type_name()
        )),
    }
}

fn parse_float_inner(value: &Value) -> Result<Value, String> {
    match value {
        Value::Float(value) => Ok(Value::Float(*value)),
        Value::Int(value) => Ok(Value::Float(*value as f64)),
        Value::Str(value) => value
            .trim()
            .parse::<f64>()
            .map(Value::Float)
            .map_err(|error| format!("parse_float: {}", error)),
        other => Err(format!(
            "parse_float: expected str/int/float, got {}",
            other.type_name()
        )),
    }
}

fn sleep_ms_inner(value: &Value) -> Result<Value, String> {
    let millis = int_arg(value, "sleep_ms: millis")?;
    if !(0..=MAX_SLEEP_MS).contains(&millis) {
        return Err(format!(
            "sleep_ms: millis must be between 0 and {}",
            MAX_SLEEP_MS
        ));
    }
    thread::sleep(Duration::from_millis(millis as u64));
    Ok(Value::Nil)
}

fn env_get_inner(value: &Value) -> Result<Value, String> {
    let name = string_arg(value, "env_get: name")?;
    match env::var_os(&name) {
        Some(value) => value
            .into_string()
            .map(|value| Value::Str(Rc::new(value)))
            .map_err(|_| format!("env_get: {} is not valid Unicode", name)),
        None => Ok(Value::Nil),
    }
}

fn chdir_inner(value: &Value) -> Result<Value, String> {
    let path = path_arg(value, "chdir: path")?;
    env::set_current_dir(&path).map_err(|error| format!("chdir: {}", error))?;
    Ok(Value::Nil)
}

fn fs_read_inner(value: &Value) -> Result<Value, String> {
    let path = path_arg(value, "fs_read: path")?;
    let metadata = fs::metadata(&path).map_err(|error| format!("fs_read: {}", error))?;
    if metadata.len() > MAX_FILE_BYTES as u64 {
        return Err(format!("fs_read: file exceeds {} bytes", MAX_FILE_BYTES));
    }
    fs::read_to_string(&path)
        .map(|body| Value::Str(Rc::new(body)))
        .map_err(|error| format!("fs_read: {}", error))
}

fn fs_write_inner(path: &Value, body: &Value) -> Result<Value, String> {
    let path = path_arg(path, "fs_write: path")?;
    let body = string_arg(body, "fs_write: body")?;
    if body.len() > MAX_FILE_BYTES {
        return Err(format!("fs_write: body exceeds {} bytes", MAX_FILE_BYTES));
    }
    fs::write(&path, body).map_err(|error| format!("fs_write: {}", error))?;
    Ok(Value::Nil)
}

fn fs_list_inner(value: &Value) -> Result<Value, String> {
    let path = path_arg(value, "fs_list: path")?;
    let mut entries = Vec::new();
    for entry in fs::read_dir(&path).map_err(|error| format!("fs_list: {}", error))? {
        let entry = entry.map_err(|error| format!("fs_list: {}", error))?;
        entries.push(Value::Str(Rc::new(
            entry.file_name().to_string_lossy().into_owned(),
        )));
    }
    entries.sort_by_key(|value| value.to_string());
    Ok(Value::List(Rc::new(RefCell::new(entries))))
}

fn fs_mkdir_inner(value: &Value) -> Result<Value, String> {
    let path = path_arg(value, "fs_mkdir: path")?;
    fs::create_dir_all(&path).map_err(|error| format!("fs_mkdir: {}", error))?;
    Ok(Value::Nil)
}

fn fs_stat_inner(value: &Value) -> Result<Value, String> {
    let path = path_arg(value, "fs_stat: path")?;
    let metadata = fs::metadata(&path).map_err(|error| format!("fs_stat: {}", error))?;
    let mut map = HashMap::new();
    map.insert("is_file".into(), Value::Bool(metadata.is_file()));
    map.insert("is_dir".into(), Value::Bool(metadata.is_dir()));
    map.insert(
        "len".into(),
        Value::Int(metadata.len().min(i64::MAX as u64) as i64),
    );
    map.insert(
        "readonly".into(),
        Value::Bool(metadata.permissions().readonly()),
    );
    map.insert(
        "modified_ms".into(),
        system_time_ms(metadata.modified().ok()),
    );
    map.insert(
        "accessed_ms".into(),
        system_time_ms(metadata.accessed().ok()),
    );
    map.insert("created_ms".into(), system_time_ms(metadata.created().ok()));
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn fs_remove_inner(value: &Value) -> Result<Value, String> {
    let path = path_arg(value, "fs_remove: path")?;
    match fs::metadata(&path) {
        Ok(metadata) if metadata.is_dir() => {
            fs::remove_dir_all(&path).map_err(|error| format!("fs_remove: {}", error))?;
            Ok(Value::Bool(true))
        }
        Ok(_) => {
            fs::remove_file(&path).map_err(|error| format!("fs_remove: {}", error))?;
            Ok(Value::Bool(true))
        }
        Err(error) if error.kind() == io::ErrorKind::NotFound => Ok(Value::Bool(false)),
        Err(error) => Err(format!("fs_remove: {}", error)),
    }
}

fn fs_rename_inner(from: &Value, to: &Value) -> Result<Value, String> {
    let from = path_arg(from, "fs_rename: from")?;
    let to = path_arg(to, "fs_rename: to")?;
    fs::rename(from, to).map_err(|error| format!("fs_rename: {}", error))?;
    Ok(Value::Nil)
}

fn fs_copy_inner(from: &Value, to: &Value) -> Result<Value, String> {
    let from = path_arg(from, "fs_copy: from")?;
    let to = path_arg(to, "fs_copy: to")?;
    let copied = fs::copy(from, to).map_err(|error| format!("fs_copy: {}", error))?;
    Ok(Value::Int(copied.min(i64::MAX as u64) as i64))
}

fn process_run_inner(args: &[Value]) -> Result<Value, String> {
    if !(1..=4).contains(&args.len()) {
        return Err("process_run expects command[, args[, stdin[, timeout_ms]]]".into());
    }

    let command = string_arg(&args[0], "process_run: command")?;
    if command.is_empty() {
        return Err("process_run: command must not be empty".into());
    }
    let command_args = match args.get(1) {
        Some(Value::Nil) | None => Vec::new(),
        Some(value) => string_list_arg(value, "process_run: args")?,
    };
    let stdin = match args.get(2) {
        Some(Value::Nil) | None => None,
        Some(value) => Some(string_arg(value, "process_run: stdin")?),
    };
    if stdin
        .as_ref()
        .map(|value| value.len() > MAX_PROCESS_IO_BYTES)
        .unwrap_or(false)
    {
        return Err(format!(
            "process_run: stdin exceeds {} bytes",
            MAX_PROCESS_IO_BYTES
        ));
    }
    let timeout = match args.get(3) {
        Some(Value::Nil) | None => DEFAULT_PROCESS_TIMEOUT_MS,
        Some(value) => timeout_arg(value)?,
    };

    let mut child = Command::new(&command)
        .args(command_args)
        .stdin(if stdin.is_some() {
            Stdio::piped()
        } else {
            Stdio::null()
        })
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("process_run: spawn {} failed: {}", command, error))?;

    let stdout = child
        .stdout
        .take()
        .ok_or_else(|| "process_run: failed to capture stdout".to_string())?;
    let stderr = child
        .stderr
        .take()
        .ok_or_else(|| "process_run: failed to capture stderr".to_string())?;
    let stdout_reader = thread::spawn(move || read_pipe_limited(stdout));
    let stderr_reader = thread::spawn(move || read_pipe_limited(stderr));

    let stdin_writer = if let Some(input) = stdin {
        let pipe = child
            .stdin
            .take()
            .ok_or_else(|| "process_run: failed to capture stdin".to_string())?;
        Some(thread::spawn(move || write_stdin(pipe, input)))
    } else {
        None
    };

    let started = Instant::now();
    let timeout = Duration::from_millis(timeout);
    let mut timed_out = false;
    let status = loop {
        if let Some(status) = child
            .try_wait()
            .map_err(|error| format!("process_run: wait failed: {}", error))?
        {
            break status;
        }
        if started.elapsed() >= timeout {
            timed_out = true;
            let _ = child.kill();
            break child
                .wait()
                .map_err(|error| format!("process_run: kill wait failed: {}", error))?;
        }
        thread::sleep(Duration::from_millis(10));
    };

    let stdout = join_reader(stdout_reader, "stdout")?;
    let stderr = join_reader(stderr_reader, "stderr")?;
    if let Some(writer) = stdin_writer {
        join_stdin_writer(writer)?;
    }
    Ok(process_result_value(status, timed_out, stdout, stderr))
}

fn process_result_value(
    status: ExitStatus,
    timed_out: bool,
    stdout: PipeOutput,
    stderr: PipeOutput,
) -> Value {
    let mut map = HashMap::new();
    map.insert(
        "success".into(),
        Value::Bool(status.success() && !timed_out),
    );
    map.insert("timed_out".into(), Value::Bool(timed_out));
    map.insert(
        "code".into(),
        status
            .code()
            .map(|code| Value::Int(code as i64))
            .unwrap_or(Value::Nil),
    );
    map.insert("stdout".into(), Value::Str(Rc::new(stdout.text)));
    map.insert("stderr".into(), Value::Str(Rc::new(stderr.text)));
    map.insert("stdout_truncated".into(), Value::Bool(stdout.truncated));
    map.insert("stderr_truncated".into(), Value::Bool(stderr.truncated));
    Value::Map(Rc::new(RefCell::new(map)))
}

fn join_reader(
    handle: thread::JoinHandle<io::Result<PipeOutput>>,
    label: &str,
) -> Result<PipeOutput, String> {
    handle
        .join()
        .map_err(|_| format!("process_run: {} reader panicked", label))?
        .map_err(|error| format!("process_run: read {} failed: {}", label, error))
}

fn join_stdin_writer(handle: thread::JoinHandle<io::Result<()>>) -> Result<(), String> {
    match handle
        .join()
        .map_err(|_| "process_run: stdin writer panicked".to_string())?
    {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == io::ErrorKind::BrokenPipe => Ok(()),
        Err(error) => Err(format!("process_run: write stdin failed: {}", error)),
    }
}

fn write_stdin(mut pipe: ChildStdin, input: String) -> io::Result<()> {
    pipe.write_all(input.as_bytes())
}

fn read_pipe_limited<R: Read>(reader: R) -> io::Result<PipeOutput> {
    let mut bytes = Vec::new();
    reader
        .take((MAX_PROCESS_IO_BYTES + 1) as u64)
        .read_to_end(&mut bytes)?;
    let truncated = bytes.len() > MAX_PROCESS_IO_BYTES;
    if truncated {
        bytes.truncate(MAX_PROCESS_IO_BYTES);
    }
    Ok(PipeOutput {
        text: String::from_utf8_lossy(&bytes).into_owned(),
        truncated,
    })
}

fn path_join_inner(value: &Value) -> Result<Value, String> {
    let parts = string_list_arg(value, "path_join: parts")?;
    let mut path = PathBuf::new();
    for part in parts {
        if !part.is_empty() {
            path.push(part);
        }
    }
    Ok(path_to_value(&normalize_path(&path)))
}

fn path_part<F>(value: &Value, label: &str, f: F) -> Result<Value, String>
where
    F: FnOnce(&Path) -> Value,
{
    let path = path_arg(value, label)?;
    Ok(f(&path))
}

fn path_resolve_inner(value: &Value) -> Result<Value, String> {
    let path = path_arg(value, "path_resolve: path")?;
    let path = if path.is_absolute() {
        path
    } else {
        env::current_dir()
            .map_err(|error| format!("path_resolve: {}", error))?
            .join(path)
    };
    Ok(path_to_value(&normalize_path(&path)))
}

fn normalize_path(path: &Path) -> PathBuf {
    let mut out = PathBuf::new();
    for component in path.components() {
        match component {
            Component::CurDir => {}
            Component::ParentDir => {
                if !out.pop() {
                    out.push("..");
                }
            }
            Component::Normal(part) => out.push(part),
            Component::RootDir | Component::Prefix(_) => out.push(component.as_os_str()),
        }
    }
    if out.as_os_str().is_empty() {
        PathBuf::from(".")
    } else {
        out
    }
}

fn path_to_value(path: &Path) -> Value {
    Value::Str(Rc::new(path.to_string_lossy().into_owned()))
}

fn base64_decode_inner(value: &Value) -> Result<Value, String> {
    let input = string_arg(value, "base64_decode: input")?;
    let bytes = base64_decode_bytes(&input)?;
    String::from_utf8(bytes)
        .map(|value| Value::Str(Rc::new(value)))
        .map_err(|error| format!("base64_decode: decoded bytes are not UTF-8: {}", error))
}

pub(crate) fn base64_encode_bytes(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 64] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut out = String::with_capacity(bytes.len().div_ceil(3) * 4);
    for chunk in bytes.chunks(3) {
        let b0 = chunk[0];
        let b1 = chunk.get(1).copied().unwrap_or(0);
        let b2 = chunk.get(2).copied().unwrap_or(0);
        out.push(ALPHABET[(b0 >> 2) as usize] as char);
        out.push(ALPHABET[(((b0 & 0x03) << 4) | (b1 >> 4)) as usize] as char);
        if chunk.len() > 1 {
            out.push(ALPHABET[(((b1 & 0x0f) << 2) | (b2 >> 6)) as usize] as char);
        } else {
            out.push('=');
        }
        if chunk.len() > 2 {
            out.push(ALPHABET[(b2 & 0x3f) as usize] as char);
        } else {
            out.push('=');
        }
    }
    out
}

pub(crate) fn base64_decode_bytes(input: &str) -> Result<Vec<u8>, String> {
    let mut sextets = Vec::new();
    for byte in input.bytes() {
        match byte {
            b'A'..=b'Z' => sextets.push(byte - b'A'),
            b'a'..=b'z' => sextets.push(byte - b'a' + 26),
            b'0'..=b'9' => sextets.push(byte - b'0' + 52),
            b'+' => sextets.push(62),
            b'/' => sextets.push(63),
            b'=' => sextets.push(64),
            b' ' | b'\n' | b'\r' | b'\t' => {}
            _ => return Err(format!("base64_decode: invalid byte 0x{byte:02x}")),
        }
    }
    if sextets.len() % 4 != 0 {
        return Err("base64_decode: input length must be a multiple of 4".into());
    }

    let mut out = Vec::with_capacity(sextets.len() / 4 * 3);
    for (block_index, block) in sextets.chunks(4).enumerate() {
        let a = block[0];
        let b = block[1];
        let c = block[2];
        let d = block[3];
        if a == 64 || b == 64 {
            return Err("base64_decode: invalid padding".into());
        }
        if c == 64 && d != 64 {
            return Err("base64_decode: invalid padding".into());
        }
        if (c == 64 || d == 64) && block_index != sextets.len() / 4 - 1 {
            return Err("base64_decode: padding is only allowed at the end".into());
        }

        out.push((a << 2) | (b >> 4));
        if c != 64 {
            out.push(((b & 0x0f) << 4) | (c >> 2));
        }
        if d != 64 {
            out.push(((c & 0x03) << 6) | d);
        }
    }
    Ok(out)
}

pub(crate) fn sha256(input: &[u8]) -> [u8; 32] {
    const K: [u32; 64] = [
        0x428a2f98, 0x71374491, 0xb5c0fbcf, 0xe9b5dba5, 0x3956c25b, 0x59f111f1, 0x923f82a4,
        0xab1c5ed5, 0xd807aa98, 0x12835b01, 0x243185be, 0x550c7dc3, 0x72be5d74, 0x80deb1fe,
        0x9bdc06a7, 0xc19bf174, 0xe49b69c1, 0xefbe4786, 0x0fc19dc6, 0x240ca1cc, 0x2de92c6f,
        0x4a7484aa, 0x5cb0a9dc, 0x76f988da, 0x983e5152, 0xa831c66d, 0xb00327c8, 0xbf597fc7,
        0xc6e00bf3, 0xd5a79147, 0x06ca6351, 0x14292967, 0x27b70a85, 0x2e1b2138, 0x4d2c6dfc,
        0x53380d13, 0x650a7354, 0x766a0abb, 0x81c2c92e, 0x92722c85, 0xa2bfe8a1, 0xa81a664b,
        0xc24b8b70, 0xc76c51a3, 0xd192e819, 0xd6990624, 0xf40e3585, 0x106aa070, 0x19a4c116,
        0x1e376c08, 0x2748774c, 0x34b0bcb5, 0x391c0cb3, 0x4ed8aa4a, 0x5b9cca4f, 0x682e6ff3,
        0x748f82ee, 0x78a5636f, 0x84c87814, 0x8cc70208, 0x90befffa, 0xa4506ceb, 0xbef9a3f7,
        0xc67178f2,
    ];

    let mut h = [
        0x6a09e667u32,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];
    let bit_len = (input.len() as u64).wrapping_mul(8);
    let mut bytes = input.to_vec();
    bytes.push(0x80);
    while bytes.len() % 64 != 56 {
        bytes.push(0);
    }
    bytes.extend_from_slice(&bit_len.to_be_bytes());

    for chunk in bytes.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (slot, word) in w.iter_mut().take(16).zip(chunk.chunks_exact(4)) {
            *slot = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }

        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        let mut f = h[5];
        let mut g = h[6];
        let mut hh = h[7];

        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let ch = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(ch)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let maj = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(maj);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
        h[5] = h[5].wrapping_add(f);
        h[6] = h[6].wrapping_add(g);
        h[7] = h[7].wrapping_add(hh);
    }

    let mut out = [0; 32];
    for (bytes, word) in out.chunks_exact_mut(4).zip(h) {
        bytes.copy_from_slice(&word.to_be_bytes());
    }
    out
}

pub(crate) fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut out = String::with_capacity(bytes.len() * 2);
    for byte in bytes {
        out.push(HEX[(byte >> 4) as usize] as char);
        out.push(HEX[(byte & 0x0f) as usize] as char);
    }
    out
}

fn url_parse_inner(value: &Value) -> Result<Value, String> {
    let input = string_arg(value, "url_parse: url")?;
    let mut rest = input.as_str();
    let mut scheme = "";
    if let Some((candidate, after_scheme)) = rest.split_once("://") {
        if !candidate.is_empty() {
            scheme = candidate;
            rest = after_scheme;
        }
    }

    let (rest_without_fragment, fragment) = split_once_optional(rest, '#');
    let authority_end = rest_without_fragment
        .char_indices()
        .find_map(|(idx, ch)| matches!(ch, '/' | '?').then_some(idx))
        .unwrap_or(rest_without_fragment.len());
    let authority = if scheme.is_empty() {
        ""
    } else {
        &rest_without_fragment[..authority_end]
    };
    let target = if scheme.is_empty() {
        rest_without_fragment
    } else {
        &rest_without_fragment[authority_end..]
    };
    let (path, query) = split_once_optional(target, '?');
    let path = if path.is_empty() && !authority.is_empty() {
        "/"
    } else {
        path
    };
    let (host, port) = parse_url_authority(authority)?;

    let mut map = HashMap::new();
    map.insert("scheme".into(), Value::Str(Rc::new(scheme.to_string())));
    map.insert(
        "authority".into(),
        Value::Str(Rc::new(authority.to_string())),
    );
    map.insert("host".into(), Value::Str(Rc::new(host.clone())));
    map.insert("port".into(), port.map(Value::Int).unwrap_or(Value::Nil));
    map.insert("path".into(), Value::Str(Rc::new(path.to_string())));
    map.insert("query".into(), optional_string_value(query));
    map.insert("fragment".into(), optional_string_value(fragment));
    map.insert(
        "origin".into(),
        if scheme.is_empty() || authority.is_empty() {
            Value::Nil
        } else {
            Value::Str(Rc::new(format!("{}://{}", scheme, authority)))
        },
    );
    Ok(Value::Map(Rc::new(RefCell::new(map))))
}

fn split_once_optional(input: &str, needle: char) -> (&str, Option<&str>) {
    match input.split_once(needle) {
        Some((left, right)) => (left, Some(right)),
        None => (input, None),
    }
}

fn parse_url_authority(authority: &str) -> Result<(String, Option<i64>), String> {
    if authority.is_empty() {
        return Ok((String::new(), None));
    }
    let without_userinfo = authority
        .rsplit_once('@')
        .map_or(authority, |(_, host)| host);
    if without_userinfo.starts_with('[') {
        let end = without_userinfo
            .find(']')
            .ok_or_else(|| "url_parse: invalid bracketed IPv6 host".to_string())?;
        let host = without_userinfo[1..end].to_string();
        let port = match without_userinfo[end + 1..].strip_prefix(':') {
            Some(port) if !port.is_empty() => Some(parse_url_port(port)?),
            Some(_) => return Err("url_parse: empty port".into()),
            None => None,
        };
        return Ok((host, port));
    }
    if without_userinfo.matches(':').count() == 1 {
        let (host, port) = without_userinfo.split_once(':').unwrap();
        if port.is_empty() {
            return Err("url_parse: empty port".into());
        }
        return Ok((host.to_string(), Some(parse_url_port(port)?)));
    }
    Ok((without_userinfo.to_string(), None))
}

fn parse_url_port(port: &str) -> Result<i64, String> {
    let port: i64 = port
        .parse()
        .map_err(|_| format!("url_parse: invalid port {}", port))?;
    if !(1..=65535).contains(&port) {
        return Err(format!("url_parse: port {} out of range", port));
    }
    Ok(port)
}

fn optional_string_value(value: Option<&str>) -> Value {
    value
        .map(|value| Value::Str(Rc::new(value.to_string())))
        .unwrap_or(Value::Nil)
}

fn string_list_value(values: impl IntoIterator<Item = String>) -> Value {
    Value::List(Rc::new(RefCell::new(
        values
            .into_iter()
            .map(|value| Value::Str(Rc::new(value)))
            .collect(),
    )))
}

fn system_time_ms(value: Option<SystemTime>) -> Value {
    value
        .and_then(|value| value.duration_since(UNIX_EPOCH).ok())
        .map(|duration| Value::Int(duration.as_millis().min(i64::MAX as u128) as i64))
        .unwrap_or(Value::Nil)
}

fn home_dir() -> Option<PathBuf> {
    if cfg!(windows) {
        env::var_os("USERPROFILE").map(PathBuf::from).or_else(|| {
            match (env::var_os("HOMEDRIVE"), env::var_os("HOMEPATH")) {
                (Some(drive), Some(path)) => {
                    let mut home = PathBuf::from(drive);
                    home.push(path);
                    Some(home)
                }
                _ => None,
            }
        })
    } else {
        env::var_os("HOME").map(PathBuf::from)
    }
}

fn timeout_arg(value: &Value) -> Result<u64, String> {
    let timeout = int_arg(value, "process_run: timeout_ms")?;
    if timeout <= 0 || timeout as u64 > MAX_PROCESS_TIMEOUT_MS {
        return Err(format!(
            "process_run: timeout_ms must be between 1 and {}",
            MAX_PROCESS_TIMEOUT_MS
        ));
    }
    Ok(timeout as u64)
}

fn string_arg(value: &Value, label: &str) -> Result<String, String> {
    match value {
        Value::Str(value) => Ok((**value).clone()),
        other => Err(format!("{} must be str, got {}", label, other.type_name())),
    }
}

fn path_arg(value: &Value, label: &str) -> Result<PathBuf, String> {
    let path = string_arg(value, label)?;
    if path.is_empty() {
        return Err(format!("{} must not be empty", label));
    }
    Ok(PathBuf::from(path))
}

fn int_arg(value: &Value, label: &str) -> Result<i64, String> {
    match value {
        Value::Int(value) => Ok(*value),
        other => Err(format!("{} must be int, got {}", label, other.type_name())),
    }
}

fn string_list_arg(value: &Value, label: &str) -> Result<Vec<String>, String> {
    let items = match value {
        Value::List(items) => items.borrow(),
        other => return Err(format!("{} must be list, got {}", label, other.type_name())),
    };
    items
        .iter()
        .enumerate()
        .map(|(index, value)| string_arg(value, &format!("{}[{}]", label, index)))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn str_value(value: &str) -> Value {
        Value::Str(Rc::new(value.into()))
    }

    fn unwrap_result(value: Value) -> Value {
        match value {
            Value::Result(result) => match result.as_ref() {
                ResultValue::Ok(value) => value.clone(),
                ResultValue::Err(error) => panic!("expected Ok, got Err({:?})", error),
            },
            other => panic!("expected Result, got {:?}", other),
        }
    }

    #[test]
    fn parses_int_and_float_as_results() {
        assert_eq!(unwrap_result(parse_int(&str_value("42"))), Value::Int(42));
        assert_eq!(
            unwrap_result(parse_float(&str_value("2.5"))),
            Value::Float(2.5)
        );
    }

    #[test]
    fn fs_tools_read_write_list_and_exists() {
        let root = env::temp_dir().join(format!(
            "tetherscript-system-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let file = root.join("hello.txt");
        let root_value = str_value(&root.to_string_lossy());
        let file_value = str_value(&file.to_string_lossy());

        unwrap_result(fs_mkdir(&root_value));
        unwrap_result(fs_write(&file_value, &str_value("hello")));
        assert_eq!(unwrap_result(fs_exists(&file_value)), Value::Bool(true));
        assert_eq!(unwrap_result(fs_read(&file_value)), str_value("hello"));

        match unwrap_result(fs_list(&root_value)) {
            Value::List(items) => {
                assert!(items.borrow().contains(&str_value("hello.txt")));
            }
            other => panic!("expected list, got {:?}", other),
        }

        let _ = fs::remove_dir_all(root);
    }

    #[test]
    fn env_get_returns_nil_for_missing_var() {
        let name = format!(
            "TETHERSCRIPT_MISSING_{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        );
        assert_eq!(unwrap_result(env_get(&str_value(&name))), Value::Nil);
    }

    #[test]
    fn process_run_captures_output_without_shell_by_default() {
        #[cfg(windows)]
        let value = process_run(&[
            str_value("cmd"),
            Value::List(Rc::new(RefCell::new(vec![
                str_value("/C"),
                str_value("echo tetherscript"),
            ]))),
        ]);

        #[cfg(not(windows))]
        let value = process_run(&[
            str_value("sh"),
            Value::List(Rc::new(RefCell::new(vec![
                str_value("-c"),
                str_value("printf tetherscript"),
            ]))),
        ]);

        match unwrap_result(value) {
            Value::Map(map) => {
                let map = map.borrow();
                assert_eq!(map.get("success"), Some(&Value::Bool(true)));
                assert!(map
                    .get("stdout")
                    .unwrap()
                    .to_string()
                    .contains("tetherscript"));
            }
            other => panic!("expected process map, got {:?}", other),
        }
    }

    #[test]
    fn path_tools_return_node_like_parts() {
        let joined = unwrap_result(path_join(&Value::List(Rc::new(RefCell::new(vec![
            str_value("alpha"),
            str_value("beta"),
            str_value("file.txt"),
        ])))));
        assert!(
            joined.to_string().ends_with("alpha\\beta\\file.txt")
                || joined.to_string().ends_with("alpha/beta/file.txt")
        );
        assert_eq!(
            unwrap_result(path_basename(&str_value("alpha/beta/file.txt"))),
            str_value("file.txt")
        );
        assert_eq!(
            unwrap_result(path_extname(&str_value("alpha/beta/file.txt"))),
            str_value(".txt")
        );
        assert_eq!(
            unwrap_result(path_normalize(&str_value("alpha/./beta/../file.txt"))),
            str_value(&PathBuf::from("alpha").join("file.txt").to_string_lossy())
        );
    }

    #[test]
    fn crypto_encoding_and_url_tools_work() {
        assert_eq!(
            unwrap_result(sha256_hex(&str_value("tetherscript"))),
            str_value("a724f07d8f90ed2c1c123a60fa8d8118f95f96dc4de19121bf91306a6bdbdb55")
        );
        let encoded = unwrap_result(base64_encode(&str_value("tetherscript")));
        assert_eq!(encoded, str_value("dGV0aGVyc2NyaXB0"));
        assert_eq!(
            unwrap_result(base64_decode(&encoded)),
            str_value("tetherscript")
        );

        match unwrap_result(url_parse(&str_value("http://example.com:8080/a?b=c#d"))) {
            Value::Map(map) => {
                let map = map.borrow();
                assert_eq!(map.get("scheme"), Some(&str_value("http")));
                assert_eq!(map.get("host"), Some(&str_value("example.com")));
                assert_eq!(map.get("port"), Some(&Value::Int(8080)));
                assert_eq!(map.get("path"), Some(&str_value("/a")));
                assert_eq!(map.get("query"), Some(&str_value("b=c")));
                assert_eq!(map.get("fragment"), Some(&str_value("d")));
            }
            other => panic!("expected URL map, got {:?}", other),
        }
    }

    #[test]
    fn fs_stat_copy_rename_and_remove_work() {
        let root = env::temp_dir().join(format!(
            "tetherscript-system-test-{}",
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ));
        let original = root.join("original.txt");
        let copied = root.join("copied.txt");
        let renamed = root.join("renamed.txt");
        let root_value = str_value(&root.to_string_lossy());
        let original_value = str_value(&original.to_string_lossy());
        let copied_value = str_value(&copied.to_string_lossy());
        let renamed_value = str_value(&renamed.to_string_lossy());

        unwrap_result(fs_mkdir(&root_value));
        unwrap_result(fs_write(&original_value, &str_value("hello")));
        assert_eq!(
            unwrap_result(fs_copy(&original_value, &copied_value)),
            Value::Int(5)
        );
        unwrap_result(fs_rename(&copied_value, &renamed_value));

        match unwrap_result(fs_stat(&renamed_value)) {
            Value::Map(map) => {
                let map = map.borrow();
                assert_eq!(map.get("is_file"), Some(&Value::Bool(true)));
                assert_eq!(map.get("len"), Some(&Value::Int(5)));
            }
            other => panic!("expected stat map, got {:?}", other),
        }

        assert_eq!(unwrap_result(fs_remove(&root_value)), Value::Bool(true));
        assert_eq!(unwrap_result(fs_exists(&root_value)), Value::Bool(false));
    }
}
