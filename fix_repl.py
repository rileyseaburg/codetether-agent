import re

with open("src/rlm/repl.rs", "r") as f:
    code = f.read()

# Replace caller 
code = code.replace(
"""            // Parse and execute commands
            if let Some(result) = self.execute_dsl_line(line) {
                match result {
                    DslResult::Output(s) => stdout.push(s),
                    DslResult::Final(s) => {
                        final_answer = Some(s);
                        break;
                    }
                    DslResult::Error(s) => stdout.push(format!("Error: {}", s)),
                }
            }""",
"""            // Parse and execute commands
            match self.execute_dsl_line(line) {
                Ok(Some(result)) => match result {
                    DslResult::Output(s) => stdout.push(s),
                    DslResult::Final(s) => {
                        final_answer = Some(s);
                        break;
                    }
                    DslResult::Error(s) => stdout.push(format!("Error: {}", s)),
                },
                Ok(None) => {}
                Err(e) => stdout.push(format!("Error: {}", e)),
            }"""
)

# Function signature
code = code.replace(
    "pub fn execute_dsl_line(&mut self, line: &str) -> Option<DslResult> {",
    "pub fn execute_dsl_line(&mut self, line: &str) -> Result<Option<DslResult>, anyhow::Error> {"
)

# Return Some to Ok(Some)
code = code.replace("return Some(DslResult::Final(answer.to_string()));", "return Ok(Some(DslResult::Final(answer.to_string())));")
code = code.replace("return Some(DslResult::Output(expanded));", "return Ok(Some(DslResult::Output(expanded)));")
code = code.replace("return Some(DslResult::Output(result));", "return Ok(Some(DslResult::Output(result)));")
code = code.replace("return None;", "return Ok(None);")

code = code.replace(
    "let start = line.find('(').unwrap() + 1;",
    "let start = line.find('(').ok_or_else(|| anyhow::anyhow!(\"missing paren\"))? + 1;"
)

# End of function None
code = code.replace(
"""        {
            let result = self.evaluate_expression(line);
            return Ok(Some(DslResult::Output(result)));
        }

        None
    }""",
"""        {
            let result = self.evaluate_expression(line);
            return Ok(Some(DslResult::Output(result)));
        }

        Ok(None)
    }"""
)

with open("src/rlm/repl.rs", "w") as f:
    f.write(code)

