import XCTest
import MediaPlayer
@testable import CodeTether

@MainActor
final class ReadbackTransportTests: XCTestCase {
    func testPauseSeekSkipAndCleanupAcrossClips() async throws {
        try ReadbackAudioSession.begin(); defer { ReadbackAudioSession.end() }
        let transport = ReadbackTransport()
        let ready = expectation(description: "three clips prepared")
        var count = 0
        let task = Task {
            try await transport.read(chunks: ["one", "two", "three"]) { _ in
                transport.pause(); count += 1
                if count == 3 { ready.fulfill() }
                return ReadbackAudioFixture.wav()
            }
        }
        await fulfillment(of: [ready], timeout: 3)
        XCTAssertEqual(transport.duration, 60, accuracy: 0.1)
        XCTAssertTrue(transport.paused)
        transport.seek(25)
        XCTAssertEqual(transport.index, 1)
        XCTAssertEqual(transport.elapsed, 25, accuracy: 0.1)
        XCTAssertTrue(transport.paused)
        transport.skip(-15)
        XCTAssertEqual(transport.index, 0)
        XCTAssertEqual(transport.elapsed, 10, accuracy: 0.1)
        transport.skip(30)
        XCTAssertEqual(transport.index, 2)
        transport.resume(); XCTAssertFalse(transport.paused)
        transport.pause(); XCTAssertTrue(transport.paused)
        let info = MPNowPlayingInfoCenter.default().nowPlayingInfo
        XCTAssertEqual(info?[MPMediaItemPropertyTitle] as? String, "CodeTether readback")
        XCTAssertEqual(info?[MPMediaItemPropertyPlaybackDuration] as? Double, 60)
        let paths = transport.cache.clips
        transport.stop()
        do { try await task.value; XCTFail("Expected cancellation") }
        catch { XCTAssertTrue(error is CancellationError) }
        XCTAssertFalse(transport.active)
        XCTAssertNil(MPNowPlayingInfoCenter.default().nowPlayingInfo)
        XCTAssertTrue(paths.allSatisfy { !FileManager.default.fileExists(atPath: $0.path) })
    }
}