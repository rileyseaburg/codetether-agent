use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader, Lines};
use tokio::process::{ChildStdin, ChildStdout};

pub fn lines(stdout: ChildStdout) -> Lines<BufReader<ChildStdout>> {
    BufReader::new(stdout).lines()
}

pub async fn write(stdin: &mut ChildStdin, command: &str) -> anyhow::Result<()> {
    stdin.write_all(command.as_bytes()).await?;
    stdin.flush().await?;
    Ok(())
}

pub async fn read_until(
    stdout: &mut Lines<BufReader<ChildStdout>>,
    marker: &str,
) -> anyhow::Result<Vec<String>> {
    let mut lines = Vec::new();
    while let Some(line) = stdout.next_line().await? {
        if line.trim() == marker {
            return Ok(lines);
        }
        lines.push(line);
    }
    anyhow::bail!("PowerShell session closed")
}
