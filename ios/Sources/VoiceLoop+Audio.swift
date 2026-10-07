import AVFoundation

/// Half-duplex recognition: never transcribe the agent's own loudspeaker output.
extension VoiceLoop {
    func appendListeningAudio(_ buffer: AVAudioPCMBuffer) {
        guard phase == .listening || phase == .userSpeaking else { return }
        transcriber?.append(buffer)
    }
}
