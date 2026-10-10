import XCTest
@testable import CodeTether

@MainActor
final class ScreenStreamRetryTests: XCTestCase {
    private let now = Date(timeIntervalSince1970: 1_000)
    private func event(_ sequence: Int) -> ScreenEvent {
        ScreenEvent(type: .snapshot, seq: sequence, text: nil, status: "ready", captured_at: nil)
    }
    func testReplayedSnapshotsCannotResetConsecutiveFailures() {
        let retry = ScreenStreamRetry(sequence: 0)
        for delay in [1, 2, 4, 8] {
            retry.receive(event(0), now: now)
            XCTAssertEqual(retry.nextDelay(after: ScreenFailure.disconnected, now: now),
                           UInt64(delay) * 1_000_000_000)
        }
        retry.receive(event(0), now: now)
        XCTAssertNil(retry.nextDelay(after: ScreenFailure.disconnected, now: now))
    }
    func testProgressAllowsMoreThanFiveSuccessfulConnections() {
        let retry = ScreenStreamRetry(sequence: 0)
        for sequence in 1...20 {
            retry.receive(event(sequence), now: now)
            XCTAssertEqual(retry.nextDelay(after: ScreenFailure.disconnected, now: now), 1_000_000_000)
        }
    }
    func testStableIdleConnectionResetsBudgetWithoutNewAnalysis() {
        let retry = ScreenStreamRetry(sequence: 0)
        for _ in 0..<20 {
            retry.receive(event(0), now: now)
            XCTAssertEqual(retry.nextDelay(after: URLError(.timedOut),
                           now: now.addingTimeInterval(31)), 1_000_000_000)
        }
    }
    func testAuthenticationAndExpiryNeverAutomaticallyRetry() {
        for code in [401, 403, 404, 410] {
            XCTAssertNil(ScreenStreamRetry(sequence: 0).nextDelay(after: ScreenFailure.http(code)))
        }
    }
}
