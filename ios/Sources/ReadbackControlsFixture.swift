#if DEBUG && targetEnvironment(simulator)
import SwiftUI
import AVFoundation

/// Simulator-only playback controls fixture, with silent local audio and no API calls.
struct ReadbackControlsFixture: View {
    @StateObject private var transport = ReadbackTransport()
    @StateObject private var loop = VoiceLoop()
    @StateObject private var mic = VoiceMicEngine()
    @State private var error = ""
    var body: some View {
        VStack {
            VoiceModeView(loop: loop, mic: mic)
            ReadbackControls(transport: transport)
            Text("\(Int(transport.elapsed))").accessibilityIdentifier("fixture-position")
            Text(error)
        }.task {
            loop.phase = .speaking
            loop.reply = String(repeating: "Scroll this long reply while the audio controls remain available. ", count: 60)
            do {
                try ReadbackAudioSession.begin()
                try await transport.read(chunks: ["one", "two", "three"]) { _ in
                    transport.pause(); return try Self.audio()
                }
            } catch { if !Task.isCancelled { self.error = "Fixture playback ended" } }
        }.onDisappear { transport.stop(); ReadbackAudioSession.end() }
    }
    static func audio() throws -> Data {
        let url = FileManager.default.temporaryDirectory.appendingPathComponent(UUID().uuidString + ".wav")
        defer { try? FileManager.default.removeItem(at: url) }
        let format = AVAudioFormat(commonFormat: .pcmFormatInt16, sampleRate: 8000, channels: 1, interleaved: false)!
        let buffer = AVAudioPCMBuffer(pcmFormat: format, frameCapacity: 240000)!
        buffer.frameLength = buffer.frameCapacity
        buffer.int16ChannelData![0].initialize(repeating: 0, count: Int(buffer.frameLength))
        do {
            let file = try AVAudioFile(forWriting: url, settings: format.settings, commonFormat: .pcmFormatInt16, interleaved: false)
            try file.write(from: buffer)
        }
        return try Data(contentsOf: url)
    }
}
#endif