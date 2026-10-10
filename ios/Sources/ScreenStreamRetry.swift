import Foundation

/// Count consecutive failures, not a lifetime limit on successful Screen connections.
@MainActor
final class ScreenStreamRetry {
    private var failures = 0
    private var sequence: Int
    private var connectedAt: Date?

    init(sequence: Int) { self.sequence = sequence }

    func receive(_ event: ScreenEvent, now: Date = Date()) {
        if connectedAt == nil { connectedAt = now }
        // Replayed snapshots alone cannot create an unbounded fast retry loop.
        if event.seq > sequence { failures = 0; sequence = event.seq }
    }

    /// Back off at 1/2/4/8 seconds, then require an explicit Reconnect.
    func nextDelay(after error: Error, now: Date = Date()) -> UInt64? {
        if let connectedAt, now.timeIntervalSince(connectedAt) >= 30 { failures = 0 }
        connectedAt = nil
        guard ScreenFailure.retryable(error) else { return nil }
        failures += 1
        guard failures < 5 else { return nil }
        return UInt64(1 << (failures - 1)) * 1_000_000_000
    }
}
