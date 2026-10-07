import AVFoundation

/// Keep built-in playback on the loudspeaker while respecting connected headphones.
@MainActor
final class VoiceAudioRouting {
    private var observer: NSObjectProtocol?

    func activate() throws {
        let session = AVAudioSession.sharedInstance()
        try session.setCategory(.playAndRecord, mode: .default,
                                options: [.defaultToSpeaker, .allowBluetooth])
        try session.setActive(true)
        try ensureSpeaker()
        if observer == nil {
            observer = NotificationCenter.default.addObserver(forName: AVAudioSession.routeChangeNotification,
                object: session, queue: .main) { [weak self] _ in
                    Task { @MainActor in try? self?.ensureSpeaker() }
                }
        }
    }

    func stop() {
        if let observer { NotificationCenter.default.removeObserver(observer) }
        observer = nil
    }

    func ensureSpeaker() throws {
        let session = AVAudioSession.sharedInstance()
        if Self.needsSpeakerOverride(session.currentRoute.outputs.map(\.portType)) {
            try session.overrideOutputAudioPort(.speaker)
        }
        PlaybackReceipt.write(phase: "voice-route", duration: 0)
    }

    static func needsSpeakerOverride(_ ports: [AVAudioSession.Port]) -> Bool {
        ports.isEmpty || ports.allSatisfy { $0 == .builtInReceiver }
    }
}