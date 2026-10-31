import AVFoundation

extension ReadbackTransport {
    func playCurrent(offset: TimeInterval = 0) throws {
        guard active, index < cache.clips.count else { return }
        let next = try AVAudioPlayer(contentsOf: cache.clips[index])
        next.delegate = self; next.volume = 1; next.currentTime = offset
        guard next.prepareToPlay() else { throw ReadbackTransportError.invalidAudio }
        player = next
        if !paused && !next.play() { throw ReadbackTransportError.invalidAudio }
        PlaybackReceipt.write(phase: paused ? "paused" : "started", duration: next.duration)
    }
    func refresh() {
        if let player { elapsed = timeline.start(of: index) + player.currentTime }
        remote.update()
    }
    nonisolated func audioPlayerDidFinishPlaying(_ player: AVAudioPlayer, successfully flag: Bool) {
        Task { @MainActor in
            guard self.player === player else { return }
            guard flag else { finish(ReadbackTransportError.invalidAudio); return }
            PlaybackReceipt.write(phase: "finished", duration: player.duration)
            elapsed = timeline.start(of: index) + player.duration
            self.player = nil; index += 1
            do {
                if index < cache.clips.count { try playCurrent() }
                else if !generating { finish() }
            } catch { finish(error) }
            refresh()
        }
    }
    nonisolated func audioPlayerDecodeErrorDidOccur(_ player: AVAudioPlayer, error: Error?) {
        Task { @MainActor in
            guard self.player === player else { return }
            finish(ReadbackTransportError.invalidAudio)
        }
    }
}





