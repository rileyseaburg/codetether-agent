//! Thread-local stdout capture for `eval()`.
//!
//! When the REPL calls `eval(source)`, we want the inner program's `print` /
//! `println` to be returned as a string, not dumped onto the host's stdout.
//! `with_capture` installs a bounded buffer on the current thread for the
//! duration of a closure; built-ins route through `write` / `writeln`, which
//! fall back to `io::stdout` outside a capture.

use std::cell::RefCell;
use std::io::{self, Write};

thread_local! {
    static CAPTURE: RefCell<Option<CaptureBuf>> = const { RefCell::new(None) };
}

struct CaptureBuf {
    buf: String,
    limit: usize,
    truncated: bool,
}

/// Run `f` with stdout captured into a string of at most `limit` bytes.
/// Returns the captured output and the closure's return value.
pub fn with_capture<F, R>(limit: usize, f: F) -> (String, R)
where
    F: FnOnce() -> R,
{
    let prev = CAPTURE.with(|c| {
        c.replace(Some(CaptureBuf {
            buf: String::new(),
            limit,
            truncated: false,
        }))
    });
    let r = f();
    let taken = CAPTURE.with(|c| c.replace(prev)).expect("capture slot");
    let mut out = taken.buf;
    if taken.truncated {
        out.push_str("\n[output truncated: exceeded capture limit]\n");
    }
    (out, r)
}

pub fn write(s: &str) {
    let wrote = CAPTURE.with(|c| {
        let mut c = c.borrow_mut();
        if let Some(b) = c.as_mut() {
            if b.truncated {
                return true;
            }
            let room = b.limit.saturating_sub(b.buf.len());
            if s.len() <= room {
                b.buf.push_str(s);
            } else {
                let mut end = room;
                // Don't split inside a UTF-8 codepoint.
                while end > 0 && !s.is_char_boundary(end) {
                    end -= 1;
                }
                b.buf.push_str(&s[..end]);
                b.truncated = true;
            }
            true
        } else {
            false
        }
    });
    if !wrote {
        let _ = io::stdout().lock().write_all(s.as_bytes());
    }
}

pub fn writeln(s: &str) {
    write(s);
    write("\n");
}
