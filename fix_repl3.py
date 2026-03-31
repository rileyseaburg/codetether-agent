with open("src/rlm/repl.rs", "r") as f:
    code = f.read()

sig = "pub fn execute_dsl_line(&mut self, line: &str) -> Option<DslResult> {"
new_sig = """pub fn execute_dsl_line(&mut self, line: &str) -> Option<DslResult> {
        match self.try_execute_dsl_line(line) {
            Ok(res) => res,
            Err(e) => Some(DslResult::Error(e.to_string()))
        }
    }

    fn try_execute_dsl_line(&mut self, line: &str) -> anyhow::Result<Option<DslResult>> {"""

code = code.replace(sig, new_sig)

# We know try_execute_dsl_line ends before find("fn expand_expression").
# Let's parse out the body.
idx_start = code.find("fn try_execute_dsl_line")
idx_end = code.find("fn expand_expression")

top = code[:idx_start]
body = code[idx_start:idx_end]
btm = code[idx_end:]

body = body.replace("let start = line.find('(').unwrap() + 1;", "let start = line.find('(').ok_or_else(|| anyhow::anyhow!(\"missing paren\"))? + 1;")
body = body.replace("return Some(", "return Ok(Some(")
body = body.replace("return None;", "return Ok(None);")

# But we need to balance the Ok(Some(...))) correctly.
body = body.replace("return Ok(Some(DslResult::Final(answer.to_string()));", "return Ok(Some(DslResult::Final(answer.to_string())));")
body = body.replace("return Ok(Some(DslResult::Output(expanded));", "return Ok(Some(DslResult::Output(expanded)));")
body = body.replace("return Ok(Some(DslResult::Output(result));", "return Ok(Some(DslResult::Output(result)));")

# Also the final None in the function
body = body.replace("\n        None\n    }", "\n        Ok(None)\n    }")

with open("src/rlm/repl.rs", "w") as f:
    f.write(top + body + btm)
