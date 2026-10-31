import MediaPlayer

extension ReadbackRemote {
    nonisolated func dispatch(_ operation: @escaping @MainActor (ReadbackTransport) -> Void) -> MPRemoteCommandHandlerStatus {
        Task { @MainActor [weak self] in
            guard let transport = self?.transport, transport.active else { return }
            operation(transport)
        }
        return .success
    }
    @objc nonisolated func playCommand(_ event: MPRemoteCommandEvent) -> MPRemoteCommandHandlerStatus {
        dispatch { $0.resume() }
    }
    @objc nonisolated func pauseCommand(_ event: MPRemoteCommandEvent) -> MPRemoteCommandHandlerStatus {
        dispatch { $0.pause() }
    }
    @objc nonisolated func toggleCommand(_ event: MPRemoteCommandEvent) -> MPRemoteCommandHandlerStatus {
        dispatch { $0.toggle() }
    }
    @objc nonisolated func backCommand(_ event: MPRemoteCommandEvent) -> MPRemoteCommandHandlerStatus {
        dispatch { $0.skip(-15) }
    }
    @objc nonisolated func forwardCommand(_ event: MPRemoteCommandEvent) -> MPRemoteCommandHandlerStatus {
        dispatch { $0.skip(15) }
    }
    @objc nonisolated func positionCommand(_ event: MPRemoteCommandEvent) -> MPRemoteCommandHandlerStatus {
        guard let position = event as? MPChangePlaybackPositionCommandEvent,
              position.positionTime.isFinite else { return .commandFailed }
        let seconds = position.positionTime
        return dispatch { $0.seek(seconds) }
    }
}