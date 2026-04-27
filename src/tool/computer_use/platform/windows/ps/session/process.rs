use tokio::io::{BufReader, Lines};
use tokio::process::{Child, ChildStdin, ChildStdout, Command};
use tokio::time::{Duration, timeout};

pub struct PsSession {
    _child: Child,
    stdin: ChildStdin,
    stdout: Lines<BufReader<ChildStdout>>,
}

impl PsSession {
    pub fn start() -> anyhow::Result<Self> {
        let mut child = Command::new("powershell.exe")
            .args(["-NoProfile", "-NonInteractive", "-Command", "-"])
            .stdin(std::process::Stdio::piped())
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .kill_on_drop(true)
            .spawn()?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("missing stdin"))?;
        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("missing stdout"))?;
        Ok(Self {
            _child: child,
            stdin,
            stdout: super::io::lines(stdout),
        })
    }

    pub async fn run(&mut self, script: &str) -> anyhow::Result<serde_json::Value> {
        let marker = format!("__CODETETHER_PS_DONE_{}__", uuid::Uuid::new_v4().simple());
        let command = crate::tool::computer_use::platform::windows::ps::parse::escaped_command(
            script, &marker,
        );
        super::io::write(&mut self.stdin, &command).await?;
        let lines = timeout(
            Duration::from_secs(30),
            super::io::read_until(&mut self.stdout, &marker),
        )
        .await
        .map_err(|_| anyhow::anyhow!("PowerShell timed out"))??;
        crate::tool::computer_use::platform::windows::ps::parse::json_line(&lines)
    }
}
