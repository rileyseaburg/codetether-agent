import MediaPlayer

/// Own only this readback's command handlers; never clear another player's state.
@MainActor
final class ReadbackRemote: NSObject {
    static weak var owner: ReadbackRemote?
    weak var transport: ReadbackTransport?
    let center = MPRemoteCommandCenter.shared()
    init(transport: ReadbackTransport) { self.transport = transport }
    func attach() {
        Self.owner?.detach(); Self.owner = self
        center.playCommand.addTarget(self, action: #selector(playCommand(_:)))
        center.pauseCommand.addTarget(self, action: #selector(pauseCommand(_:)))
        center.togglePlayPauseCommand.addTarget(self, action: #selector(toggleCommand(_:)))
        center.skipBackwardCommand.addTarget(self, action: #selector(backCommand(_:)))
        center.skipForwardCommand.addTarget(self, action: #selector(forwardCommand(_:)))
        center.changePlaybackPositionCommand.addTarget(self, action: #selector(positionCommand(_:)))
        center.skipBackwardCommand.preferredIntervals = [15]
        center.skipForwardCommand.preferredIntervals = [15]
        for command in commands { command.isEnabled = true }
        update()
    }
    var commands: [MPRemoteCommand] {
        [center.playCommand, center.pauseCommand, center.togglePlayPauseCommand,
         center.skipBackwardCommand, center.skipForwardCommand, center.changePlaybackPositionCommand]
    }
    func detach() {
        for command in commands { command.removeTarget(self) }
        guard Self.owner === self else { return }
        for command in commands { command.isEnabled = false }
        MPNowPlayingInfoCenter.default().nowPlayingInfo = nil; Self.owner = nil
    }
    func update() {
        guard Self.owner === self, let transport, transport.active else { return }
        MPNowPlayingInfoCenter.default().nowPlayingInfo = [MPMediaItemPropertyTitle: "CodeTether readback",
            MPMediaItemPropertyPlaybackDuration: transport.duration,
            MPNowPlayingInfoPropertyElapsedPlaybackTime: transport.elapsed,
            MPNowPlayingInfoPropertyPlaybackRate: transport.player?.isPlaying == true ? 1.0 : 0.0]
    }
}