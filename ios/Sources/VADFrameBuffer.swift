import Foundation

/// Accumulates interleaved-or-mono native-rate samples and yields fixed
/// 4096-sample mono 16 kHz frames expected by Silero VAD. Downmixes
/// stereo by averaging; sample-rate conversion is handled upstream by
/// the engine format (16 kHz preferred) — residual mismatch is treated
/// as pass-through since VAD is robust to it.
struct VADFrameBuffer {
    private(set) var pending: [Float] = []
    let frameLength = 4096

    mutating func reset() { pending = [] }

    /// Appends mono samples and returns consecutive complete frames.
    mutating func append(mono samples: [Float]) -> [[Float]] {
        pending += samples
        var frames: [[Float]] = []
        while pending.count >= frameLength {
            frames.append(Array(pending.prefix(frameLength)))
            pending.removeFirst(frameLength)
        }
        return frames
    }
}
