import XCTest
@testable import CodeTether

final class SpeechReadbackCancellationTests: XCTestCase {
    @MainActor
    func testStopCancelsPlaybackAndPrefetchedRequest() async {
        let fetching = expectation(description: "prefetch started")
        var cancelled = false, played = 0
        let task = Task {
            try await SpeechReadback.run(["one", "two", "three"], synthesize: { text in
                if text == "two" {
                    fetching.fulfill()
                    do { try await Task.sleep(nanoseconds: 60_000_000_000) }
                    catch { cancelled = true; throw error }
                }
                return Data(text.utf8)
            }, play: { _ in
                played += 1
                try await Task.sleep(nanoseconds: 60_000_000_000)
            })
        }
        await fulfillment(of: [fetching], timeout: 2)
        task.cancel()
        do { try await task.value; XCTFail("expected cancellation") }
        catch { XCTAssertTrue(error is CancellationError) }
        XCTAssertTrue(cancelled)
        XCTAssertEqual(played, 1)
    }
    @MainActor
    func testPlaybackFailureCancelsPendingPrefetch() async {
        let fetching = expectation(description: "prefetch started")
        var cancelled = false
        do {
            try await SpeechReadback.run(["one", "two"], synthesize: { text in
                if text == "two" {
                    fetching.fulfill()
                    do { try await Task.sleep(nanoseconds: 60_000_000_000) }
                    catch { cancelled = true; throw error }
                }
                return Data(text.utf8)
            }, play: { _ in
                await self.fulfillment(of: [fetching], timeout: 2)
                throw ClientError.invalidResponse
            })
        } catch { XCTAssertTrue(cancelled) }
    }
}