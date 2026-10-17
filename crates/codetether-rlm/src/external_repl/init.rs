//! Generate the original JavaScript context helpers and framing.
use super::*;

impl ExternalRepl {
    pub(super) fn generate_bun_init(context: &str) -> String {
        let escaped = context
            .replace('\\', "\\\\")
            .replace('"', "\\\"")
            .replace('\n', "\\n");

        format!(
            r#"
const readline = require('readline');
const rl = readline.createInterface({{ input: process.stdin, output: process.stdout, terminal: false }});

const context = "{escaped}";

function lines() {{ return context.split("\n"); }}
function head(n = 10) {{ return lines().slice(0, n).join("\n"); }}
function tail(n = 10) {{ return lines().slice(-n).join("\n"); }}
function grep(pattern) {{
    const re = pattern instanceof RegExp ? pattern : new RegExp(pattern, 'gi');
    return lines().filter(l => re.test(l));
}}
function count(pattern) {{
    const re = pattern instanceof RegExp ? pattern : new RegExp(pattern, 'gi');
    return (context.match(re) || []).length;
}}
function FINAL(answer) {{
    console.log("__FINAL__" + String(answer) + "__FINAL_END__");
}}

console.log("READY");

rl.on('line', async (line) => {{
    try {{
        const result = eval(line);
        if (result !== undefined) console.log(result);
    }} catch (e) {{
        console.error("Error:", e.message);
    }}
    console.log("__DONE__");
}});
"#
        )
    }
}
