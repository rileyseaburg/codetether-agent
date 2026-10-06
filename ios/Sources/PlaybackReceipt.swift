import AVFoundation

enum PlaybackReceipt {
    private struct Receipt: Encodable {
        let source = "Kokoro via https://server.codetether.run/tts/speak"
        let phase: String
        let duration: Double
        let volume: Float
        let routes: [String]
        let checkedAt = Date()
    }
    static func write(phase: String, duration: Double) {
        let session = AVAudioSession.sharedInstance()
        let value = Receipt(phase: phase, duration: duration, volume: session.outputVolume,
                            routes: session.currentRoute.outputs.map { $0.portType.rawValue })
        let encoder = JSONEncoder()
        encoder.dateEncodingStrategy = .iso8601
        if let data = try? encoder.encode(value) {
            try? data.write(to: URL.documentsDirectory.appendingPathComponent("kokoro-playback-\(phase).json"),
                            options: [.atomic, .completeFileProtection])
        }
    }
}
