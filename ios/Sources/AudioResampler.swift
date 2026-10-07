import AVFoundation
import Foundation

/// Resamples native-rate mic buffers to 16 kHz mono for Silero VAD.
struct AudioResampler {
    let source: AVAudioFormat
    let target = AVAudioFormat(commonFormat: .pcmFormatFloat32, sampleRate: 16_000,
                               channels: 1, interleaved: false)!

    private let converter: AVAudioConverter

    init?(from source: AVAudioFormat) {
        guard let converter = AVAudioConverter(from: source, to: target) else { return nil }
        self.source = source
        self.converter = converter
    }

    /// Converts one buffer; drops nothing (input under-run yields short output).
    /// Capacity scales with input length so 24 kHz native input (which
    /// `.voiceChat` often forces) never truncates: 2048 frames @ 24 kHz →
    /// ~1366 output frames, exceeding a fixed 1024 budget.
    func convert(_ buffer: AVAudioPCMBuffer) -> [Float] {
        let capacity = AVAudioFrameCount(Double(buffer.frameLength) * 16_000 / source.sampleRate) + 64
        guard let out = AVAudioPCMBuffer(pcmFormat: target, frameCapacity: capacity) else { return [] }
        var fed = false
        var error: NSError?
        converter.convert(to: out, error: &error) { _, status in
            if fed {
                status.pointee = .noDataNow
                return nil
            }
            fed = true
            status.pointee = .haveData
            return buffer
        }
        guard error == nil, let data = out.floatChannelData?[0] else { return [] }
        return Array(UnsafeBufferPointer(start: data, count: Int(out.frameLength)))
    }
}
