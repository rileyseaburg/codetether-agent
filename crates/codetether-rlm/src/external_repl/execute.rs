//! Exchange one command with the child and collect framed output.
use super::*;
use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::time::timeout;
impl ExternalRepl {
    /// Execute code and get result
    ///
    /// # Arguments
    /// * `code` - JavaScript command written to the child process.
    /// # Returns
    /// Output collected until a done marker, final-answer marker, EOF, or timeout.
    /// # Errors
    /// Returns an error when pipes are missing or I/O fails.
    /// # Panics
    /// Malformed final-answer markers may yield an invalid slice boundary.
    /// # Examples
    /// ```no_run
    /// # async fn example(repl: &mut codetether_rlm::external_repl::ExternalRepl) -> anyhow::Result<()> {
    /// let result = repl.execute("FINAL('done')").await?;
    /// assert_eq!(result.final_answer.as_deref(), Some("done"));
    /// # Ok(()) }
    /// ```
    pub async fn execute(&mut self, code: &str) -> Result<ReplResult> {
        let stdin = self
            .child
            .stdin
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("No stdin"))?;
        let stdout = self
            .child
            .stdout
            .as_mut()
            .ok_or_else(|| anyhow::anyhow!("No stdout"))?;

        stdin.write_all(code.as_bytes()).await?;
        stdin.write_all(b"\n").await?;
        stdin.flush().await?;

        let mut reader = BufReader::new(stdout);
        let mut output = Vec::new();
        let mut final_answer = None;

        loop {
            let mut line = String::new();
            match timeout(Duration::from_secs(30), reader.read_line(&mut line)).await {
                Ok(Ok(0)) | Err(_) => break, // EOF or timeout
                Ok(Ok(_)) => {
                    let line = line.trim();
                    if line == "__DONE__" {
                        break;
                    }
                    if let Some(answer) = Self::extract_final(line) {
                        final_answer = Some(answer);
                        break;
                    }
                    output.push(line.to_string());
                }
                Ok(Err(e)) => return Err(anyhow::anyhow!("Read error: {}", e)),
            }
        }

        Ok(ReplResult {
            stdout: output.join("\n"),
            stderr: String::new(),
            final_answer,
        })
    }
}
