import Foundation

extension ReadbackTransport {
    func pause() {
        guard active else { return }
        paused = true; player?.pause(); refresh()
    }
    func resume() {
        guard active else { return }
        paused = false
        if let player, !player.play() { finish(ReadbackTransportError.invalidAudio) }
        refresh()
    }
    func toggle() { if paused { resume() } else { pause() } }
    func skip(_ seconds: TimeInterval) { refresh(); seek(elapsed + seconds) }
    func seek(_ seconds: TimeInterval) {
        guard active, let position = timeline.position(at: seconds) else { return }
        player?.stop(); player = nil
        index = position.index; elapsed = timeline.start(of: index) + position.offset
        if elapsed >= duration && !generating { finish(); return }
        do { try playCurrent(offset: min(position.offset, timeline.durations[index] - 0.001)) }
        catch { finish(error) }
        refresh()
    }
}



