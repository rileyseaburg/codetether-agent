import AVFoundation

/// Chat owns a session for the entire readback, rather than for each chunk.
/// Voice mode leaves session ownership with its microphone engine.
@MainActor
enum ReadbackAudioSession {
    static func begin() throws {
        let session = AVAudioSession.sharedInstance()
        try session.setCategory(.playback, mode: .spokenAudio)
        try session.setActive(true)
    }
    static func end() {
        try? AVAudioSession.sharedInstance().setActive(false, options: .notifyOthersOnDeactivation)
    }
}