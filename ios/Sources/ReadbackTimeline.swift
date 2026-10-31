import Foundation

/// Maps one continuous seek position onto separately generated speech clips.
struct ReadbackTimeline {
    var durations: [TimeInterval] = []
    var duration: TimeInterval { durations.reduce(0, +) }
    func start(of index: Int) -> TimeInterval { durations.prefix(index).reduce(0, +) }
    func position(at seconds: TimeInterval) -> (index: Int, offset: TimeInterval)? {
        guard seconds.isFinite, !durations.isEmpty else { return nil }
        let target = min(max(seconds, 0), duration)
        var start: TimeInterval = 0
        for (index, length) in durations.enumerated() {
            if target < start + length || index == durations.count - 1 {
                return (index, min(max(target - start, 0), length))
            }
            start += length
        }
        return nil
    }
}

enum ReadbackTransportError: Error { case invalidAudio, cacheLimit }
