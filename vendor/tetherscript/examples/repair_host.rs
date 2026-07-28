//! Rust host that drives the `repair_msg` TetherScript plugin.
//!
//! Usage:
//!   cargo run --example repair_host messages.json repaired.json examples/repair_msg.tether
//!
//! The host owns all IO and iteration. The `.tether` script only defines
//! `fn repair_msg(msg) -> msg` — a pure transformation with no capability
//! grants needed.
//!
//! # Architecture
//!
//! ```text
//! Rust host: read JSON → parse → iterate messages → write output
//!                                ↕
//! TetherScript plugin: repair_msg(msg) → msg  (pure transform)
//! ```
//!
//! The script receives *no* capability grants (no fs, no json_parse, no network).
//! All IO stays in the Rust host.

use std::cell::RefCell;
use std::env;
use std::fs;
use std::rc::Rc;

use tetherscript::json;
use tetherscript::plugin::PluginHost;
use tetherscript::value::Value;

fn main() -> Result<(), String> {
    let args: Vec<String> = env::args().collect();
    if args.len() != 4 {
        eprintln!("usage: repair_host <input.json> <output.json> <script.tether>");
        std::process::exit(1);
    }
    let in_path = &args[1];
    let out_path = &args[2];
    let script_path = &args[3];

    // --- 1. Read and parse JSON in Rust ---
    let raw = fs::read_to_string(in_path).map_err(|e| format!("read {}: {}", in_path, e))?;
    let data =
        json::parse(&Value::Str(Rc::new(raw))).map_err(|e| format!("parse {}: {}", in_path, e))?;

    let messages = match data {
        Value::List(list) => list,
        other => return Err(format!("expected JSON array, got {}", other.type_name())),
    };

    let total = messages.borrow().len();

    // --- 2. Load the TetherScript plugin (no capability grants needed) ---
    let host = PluginHost::new();
    let mut plugin = host.load_file(script_path).map_err(|e| e.to_string())?;

    if !plugin.has_hook("repair_msg") {
        return Err(format!(
            "{}: missing `fn repair_msg(msg)` hook",
            script_path
        ));
    }

    // --- 3. Iterate in Rust, call repair_msg per message ---
    //
    // Note: TetherScript maps are Rc<RefCell<HashMap>>, so the script's
    // `msg.content = repaired` mutates in-place. We collect the returned
    // values to build the output array.
    let mut results = Vec::with_capacity(total);

    for (i, msg) in messages.borrow().iter().enumerate() {
        let call = plugin
            .call("repair_msg", &[msg.clone()])
            .map_err(|e| format!("repair_msg #{}: {}", i + 1, e))?;

        results.push(call.value);
    }

    // --- 4. Re-encode and write ---
    let result_array = Value::List(Rc::new(RefCell::new(results)));
    let json_out = json::encode_pretty(&result_array).map_err(|e| format!("encode: {}", e))?;

    let json_str = match json_out {
        Value::Str(s) => s.to_string(),
        other => return Err(format!("json_encode returned {}", other.type_name())),
    };

    fs::write(out_path, &json_str).map_err(|e| format!("write {}: {}", out_path, e))?;

    println!("processed {} messages: {} -> {}", total, in_path, out_path);

    Ok(())
}
