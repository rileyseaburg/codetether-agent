# TetherScript Standard Tools

TetherScript's trusted CLI/runtime should include these scripting primitives. Host tools are intentionally not installed in the sandboxed `eval()` runtime. The language crate is zero-dependency: all standard tools are implemented with Rust `std` and in-tree code.

## Core Values

- `print(...)`, `println(...)`: write text output.
- `len(value)`: length for strings, lists, and maps.
- `type_of(value)`: return TetherScript's runtime type name.
- `str(value)`: convert any value to its display string.
- `parse_int(value)`, `parse_float(value)`: parse numeric text and return `Result`.
- `assert(condition[, message])`: stop execution when an invariant is false.
- `sha256_hex(text)`: compute a SHA-256 hex digest with TetherScript's std-only implementation.
- `base64_encode(text)`, `base64_decode(text)`: encode/decode Base64 text with TetherScript's std-only implementation.
- `Ok(value)`, `Err(message)`: construct `Result` values for `?` propagation.
- `map()`: construct an empty map.

## Collections And Text

- String methods: `.len()`, `.upper()`, `.lower()`, `.trim()`, `.contains(s)`, `.starts_with(s)`, `.ends_with(s)`, `.replace(from, to)`, `.split(sep)`, `.lines()`.
- List methods: `.len()`, `.push(value)`, `.pop()`, `.join(sep)`, `.contains(value)`.
- Map methods: `.len()`, `.keys()`, `.values()`, `.contains(key)`.

## Data Formats

- `json_parse(text)`: parse JSON into TetherScript values.
- `json_encode(value)`: encode compact JSON.
- `json_encode_pretty(value)`: encode formatted JSON.

## Host Runtime

- Time: `time_now_ms()`, `sleep_ms(ms)`.
- Environment: `env_get(name)`.
- Process metadata: `process_args()`, `process_pid()`, `process_platform()`, `process_arch()`.
- OS metadata: `os_platform()`, `os_arch()`, `os_tmpdir()`, `os_homedir()`, `os_eol()`.
- Working directory: `cwd()`, `chdir(path)`.
- Filesystem: `fs_read(path)`, `fs_write(path, body)`, `fs_exists(path)`, `fs_list(path)`, `fs_mkdir(path)`, `fs_stat(path)`, `fs_remove(path)`, `fs_rename(from, to)`, `fs_copy(from, to)`.
- Path: `path_sep()`, `path_join(parts)`, `path_dirname(path)`, `path_basename(path)`, `path_extname(path)`, `path_normalize(path)`, `path_resolve(path)`.
- URL: `url_parse(url)`.
- Process execution: `process_run(command[, args[, stdin[, timeout_ms]]])`.
- HTTP: `http_get(url)`, `http_head(url)`, `http_post(url, body)`, `http_request(method, url[, body[, headers]])`, `http_serve(port, handler)`, `http_serve_static(port, root_dir)`.
- Browser runtime: `browser_parse_html(html)`, `browser_parse_css(css)`, `browser_styles(html[, css])`, `browser_query_selector(html, selector)`, `browser_text_content(html, selector)`, `browser_snapshot(html[, css[, width]])`, `browser_layout(html[, css[, width]])`, `browser_display_list(html[, css[, width]])`, `browser_render(html[, css[, width]])`, `js_eval(source)`, `browser_eval_js(html, script)`, `browser_run_scripts(html)`, `browser_compatibility_report()`. The browser JS host exposes DOM querying/mutation, synchronous event listeners/property handlers, `this`, `typeof`, function expressions, `location`, and `navigator`.
- SMTP: `smtp_send(host, port, from, to, subject, body)`.

## Safety Defaults

- Fallible host tools return TetherScript `Result` values so scripts can use `?`, `.unwrap()`, `.is_ok()`, and `.err()`.
- `process_run` executes a command plus argv directly; it does not invoke a shell unless the script explicitly runs one.
- File and process output are bounded to keep accidental large reads from consuming unbounded memory.
- Std-only HTTP supports plain `http://`; `https://` requires TLS and is rejected explicitly.
- The standard-tool implementation itself uses Rust `std` only. It does not add or depend on external crates for JSON, LSP JSON-RPC framing, SMTP DKIM signing, path, filesystem, process, Base64, SHA-256, or URL parsing.
