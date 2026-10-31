import XCTest
@testable import CodeTether

final class ReadbackTimelineTests: XCTestCase {
    func testSeeksAcrossChunkBoundariesAndClamps() {
        let timeline = ReadbackTimeline(durations: [10, 20, 15])
        XCTAssertEqual(timeline.duration, 45)
        XCTAssertEqual(timeline.position(at: 10)?.index, 1)
        XCTAssertEqual(timeline.position(at: 10)?.offset, 0)
        XCTAssertEqual(timeline.position(at: 35)?.index, 2)
        XCTAssertEqual(timeline.position(at: 35)?.offset, 5)
        XCTAssertEqual(timeline.position(at: -15)?.offset, 0)
        XCTAssertEqual(timeline.position(at: 100)?.offset, 15)
        XCTAssertEqual(timeline.start(of: 2), 30)
        XCTAssertNil(timeline.position(at: .nan))
        XCTAssertNil(timeline.position(at: .infinity))
        XCTAssertNil(ReadbackTimeline().position(at: 1))
    }
    func testCacheDurationAndDiscard() throws {
        let cache = ReadbackClipCache()
        XCTAssertEqual(try cache.append(ReadbackAudioFixture.wav()), 20, accuracy: 0.01)
        let clip = try XCTUnwrap(cache.clips.first)
        XCTAssertTrue(FileManager.default.fileExists(atPath: clip.path))
        cache.discard()
        XCTAssertFalse(FileManager.default.fileExists(atPath: clip.path))
        XCTAssertTrue(cache.clips.isEmpty)
    }
}