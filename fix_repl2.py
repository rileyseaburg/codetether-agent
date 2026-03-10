import re

with open("src/rlm/repl.rs", "r") as f:
    code = f.read()

# Replace the method signature and wrap
orig_sig = "pub fn execute_dsl_line(&mut self, line: &str) -> Option<DslResult> {"
new_sig = """pub fn execute_dsl_line(&mut self, line: &str) -> Option<DslResult> {
        match self.try_execute_dsl_line(line) {
            Ok(res) => res,
            Err(e) => Some(DslResult::Error(e.to_string()))
        }
    }

    fn try_execute_dsl_line(&mut self, line: &str) -> anyhow::Result<Option<DslResult>> {"""

code = code.replace(orig_sig, new_sig)

# Replace target unwraps
code = code.replace(
    "let start = line.find('(').unwrap() + 1;",
    "let start = line.find('(').ok_or_else(|| anyhow::anyhow!(\"missing paren\"))? + 1;"
)

# Return Some to Ok(Some) inside the new function
# We must only replace the ones inside try_execute_dsl_line.
# Let's just do a string replace of the block.
code = code.replace(
"""            return Some(DslResult::Final(answer.to_string()));
        }

        // Check for print/console.log
        if line.starts_with("print(")
            || line.starts_with("println!(")
            || line.starts_with("console.log(")
        {
            let start = line.find('(').ok_or_else(|| anyhow::anyhow!("missing paren"))? + 1;
            let end = line.rfind(')').unwrap_or(line.len());
            let content = line[start..end]
                .trim()
                .trim_matches(|c| c == '"' || c == '\'' || c == '`');

            // Expand variables
            let expanded = self.expand_expression(content);
            return Some(DslResult::Output(expanded));
        }

        // Check for variable assignment
        if let Some(eq_pos) = line.find('=')
            && !line.contains("==")
            && !line.starts_with("if ")
        {
            let var_name = line[..eq_pos]
                .trim()
                .trim_start_matches("let ")
                .trim_start_matches("const ")
                .trim_start_matches("var ")
                .trim();
            let expr = line[eq_pos + 1..].trim().trim_end_matches(';');

            let value = self.evaluate_expression(expr);
            self.set_var(var_name, value);
            return None;
        }

        // Check for function calls that should output
        if line.starts_with("head(")
            || line.starts_with("tail(")
            || line.starts_with("grep(")
            || line.starts_with("count(")
            || line.starts_with("lines(")
            || line.starts_with("slice(")
            || line.starts_with("chunks(")
            || line.starts_with("ast_query(")
            || line.starts_with("context")
        {
            let result = self.evaluate_expression(line);
            return Some(DslResult::Output(result));
        }

        None
    }""",
"""            return Ok(Some(DslResult::Final(answer.to_string())));
        }

        // Check for print/console.log
        if line.starts_with("print(")
            || line.starts_with("println!(")
            || line.starts_with("console.log(")
        {
            let start = line.find('(').ok_or_else(|| anyhow::anyhow!("missing paren"))? + 1;
            let end = line.rfind(')').unwrap_or(line.len());
            let content = line[start..end]
                .trim()
                .trim_matches(|c| c == '"' || c == '\'' || c == '`');

            // Expand variables
            let expanded = self.expand_expression(content);
            return Ok(Some(DslResult::Output(expanded)));
        }

        // Check for variable assignment
        if let Some(eq_pos) = line.find('=')
            && !line.contains("==")
            && !line.starts_with("if ")
        {
            let var_name = line[..eq_pos]
                .trim()
                .trim_start_matches("let ")
                .trim_start_matches("const ")
                .trim_start_matches("var ")
                .trim();
            let expr = line[eq_pos + 1..].trim().trim_end_matches(';');

            let value = self.evaluate_expression(expr);
            self.set_var(var_name, value);
            return Ok(None);
        }

        // Check for function calls that should output
        if line.starts_with("head(")
            || line.starts_with("tail(")
            || line.starts_with("grep(")
            || line.starts_with("count(")
            || line.starts_with("lines(")
            || line.starts_with("slice(")
            || line.starts_with("chunks(")
            || line.starts_with("ast_query(")
            || line.starts_with("context")
        {
            let result = self.evaluate_expression(line);
            return Ok(Some(DslResult::Output(result)));
        }

        Ok(None)
    }"""
)

with open("src/rlm/repl.rs", "w") as f:
    f.write(code)

