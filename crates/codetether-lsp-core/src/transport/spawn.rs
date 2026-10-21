//! spawn responsibility for the stdio LSP transport.

use super::*;

impl LspTransport {
    /// Spawn a language server and create a transport
    pub async fn spawn(command: &str, args: &[String], timeout_ms: u64) -> Result<Self> {
        let mut child = process::spawn(command, args)?;

        let stdout = child
            .stdout
            .take()
            .ok_or_else(|| anyhow::anyhow!("No stdout"))?;
        let stderr = child
            .stderr
            .take()
            .ok_or_else(|| anyhow::anyhow!("No stderr"))?;
        let stdin = child
            .stdin
            .take()
            .ok_or_else(|| anyhow::anyhow!("No stdin"))?;

        let (write_tx, write_rx) = mpsc::channel::<String>(100);
        let pending: Arc<RwLock<HashMap<i64, oneshot::Sender<JsonRpcResponse>>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let recent_stderr = Arc::new(RwLock::new(Vec::new()));
        let diagnostics = Arc::new(RwLock::new(HashMap::new()));
        let diag_publish_seq = Arc::new(AtomicU64::new(0));
        let reader_alive = Arc::new(std::sync::atomic::AtomicBool::new(true));

        writer::start(stdin, write_rx, Arc::clone(&pending));
        stderr::start(stderr, Arc::clone(&recent_stderr), command.to_string());
        reader::start(
            stdout,
            Arc::clone(&pending),
            Arc::clone(&diagnostics),
            Arc::clone(&diag_publish_seq),
            write_tx.clone(),
            Arc::clone(&reader_alive),
        );

        Ok(Self {
            _child: child,
            tx: write_tx,
            pending,
            request_id: AtomicI64::new(1),
            initialized: std::sync::atomic::AtomicBool::new(false),
            timeout_ms,
            recent_stderr,
            command: command.to_string(),
            diagnostics,
            diag_publish_seq,
            sharing: std::sync::Mutex::new(None),
            reader_alive,
        })
    }
}
