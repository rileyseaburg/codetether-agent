import AVFoundation

@MainActor
final class AudioPlayback: NSObject, AVAudioPlayerDelegate {
    private var player: AVAudioPlayer?
    private var completion: CheckedContinuation<Void, Error>?
    func play(_ data: Data) async throws {
        let session = AVAudioSession.sharedInstance()
        try session.setCategory(.playback, mode: .spokenAudio)
        try session.setActive(true)
        let active = try AVAudioPlayer(data: data)
        player = active
        active.delegate = self
        guard active.prepareToPlay() else { throw ClientError.invalidResponse }
        try await withTaskCancellationHandler {
            try await withCheckedThrowingContinuation { continuation in
                completion = continuation
                if player?.play() == true {
                    PlaybackReceipt.write(phase: "started", duration: player?.duration ?? 0)
                } else { finish(ClientError.invalidResponse) }
            }
        } onCancel: {
            Task { @MainActor in if self.player === active { self.stop() } }
        }
    }
    func stop() {
        guard player != nil else { return }
        player?.stop(); finish(CancellationError())
    }
    private func finish(_ error: Error? = nil) {
        if let error { completion?.resume(throwing: error) }
        else { completion?.resume() }
        completion = nil
        player = nil
        try? AVAudioSession.sharedInstance().setActive(false, options: .notifyOthersOnDeactivation)
    }
    nonisolated func audioPlayerDidFinishPlaying(_ player: AVAudioPlayer, successfully flag: Bool) {
        let duration = player.duration
        Task { @MainActor in
            if flag { PlaybackReceipt.write(phase: "finished", duration: duration) }
            finish(flag ? nil : ClientError.invalidResponse)
        }
    }
    nonisolated func audioPlayerDecodeErrorDidOccur(_ player: AVAudioPlayer, error: Error?) {
        Task { @MainActor in finish(ClientError.invalidResponse) }
    }
}