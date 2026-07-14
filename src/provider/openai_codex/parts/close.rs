impl<S> OpenAiRealtimeConnection<S>
where
    S: AsyncRead + AsyncWrite + Unpin,
{
    /// Closes the WebSocket connection idempotently.
    ///
    /// # Errors
    ///
    /// Returns an error for close failures other than an already-closed or
    /// disconnected transport.
    pub async fn close(&mut self) -> Result<()> {
        match self.stream.send(WsMessage::Close(None)).await {
            Ok(()) => {}
            Err(WsError::ConnectionClosed) | Err(WsError::AlreadyClosed) => {}
            Err(WsError::Io(err))
                if matches!(
                    err.kind(),
                    ErrorKind::BrokenPipe
                        | ErrorKind::ConnectionReset
                        | ErrorKind::ConnectionAborted
                        | ErrorKind::NotConnected
                ) => {}
            Err(err) => return Err(err).context("Failed to close Realtime WebSocket"),
        }
        Ok(())
    }
}
