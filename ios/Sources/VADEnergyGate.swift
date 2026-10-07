import Foundation

/// Log-domain FIFO energy meter for the orb's activity animation.
/// Also used as the pre-VAD gate: frames below the noise floor are
/// clamped to probability 0 before the hysteresis detector.
struct VADEnergyGate {
    private var window: [Float] = []
    let size: Int
    let floor: Float

    /// - Parameter floor: dBFS level below which input is treated as silence.
    init(windowSize: Int = 12, floor: Float = -45) {
        size = windowSize
        self.floor = floor
    }

    /// RMS energy in dBFS of one 4096-sample frame.
    static func levelDb(_ frame: [Float]) -> Float {
        guard !frame.isEmpty else { return -120 }
        var sum: Double = 0
        for s in frame { sum += Double(s) * Double(s) }
        return 20 * log10(max(Float((sum / Double(frame.count)).squareRoot()), 1e-6))
    }

    /// True when recent energy is above the noise floor.
    var isOpen: Bool { window.last.map { $0 > floor } ?? false }

    /// Pushes one frame's dBFS level; returns updated `isOpen`.
    mutating func push(_ levelDb: Float) -> Bool {
        window.append(levelDb)
        if window.count > size { window.removeFirst(window.count - size) }
        return isOpen
    }
}
