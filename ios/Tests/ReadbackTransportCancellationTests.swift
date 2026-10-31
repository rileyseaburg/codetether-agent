import XCTest
@testable import CodeTether

@MainActor
final class ReadbackTransportCancellationTests: XCTestCase {
    func testStopCancelsInFlightSynthesis() async {
        let transport = ReadbackTransport()
        let started = expectation(description: "synthesis started")
        let cancelled = expectation(description: "synthesis cancelled")
        let task = Task {
            try await transport.read(chunks: ["one"]) { _ in
                started.fulfill()
                do { try await Task.sleep(nanoseconds: 60_000_000_000) }
                catch { cancelled.fulfill(); throw error }
                return ReadbackAudioFixture.wav()
            }
        }
        await fulfillment(of: [started], timeout: 3)
        transport.stop()
        do { try await task.value; XCTFail("Expected cancellation") }
        catch { XCTAssertTrue(error is CancellationError) }
        await fulfillment(of: [cancelled], timeout: 3)
        XCTAssertTrue(transport.cache.clips.isEmpty)
    }
    func testInvalidAudioFailsWithoutLeavingRemotePlaybackActive() async {
        let transport = ReadbackTransport()
        do {
            try await transport.read(chunks: ["one"]) { _ in Data("not WAV".utf8) }
            XCTFail("Expected invalid audio")
        } catch { XCTAssertFalse(transport.active) }
        XCTAssertTrue(transport.cache.clips.isEmpty)
    }
    func testEmptyReadbackDoesNotCallSynthesis() async throws {
        let transport = ReadbackTransport()
        try await transport.read(chunks: []) { _ in XCTFail("Unexpected synthesis"); return Data() }
        XCTAssertFalse(transport.active)
    }
}
