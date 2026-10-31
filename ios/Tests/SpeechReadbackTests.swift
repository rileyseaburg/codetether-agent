import XCTest
@testable import CodeTether

final class SpeechReadbackTests: XCTestCase {
    @MainActor
    func testPreparesNextChunkDuringPlaybackInOrderWithBoundedLookahead() async throws {
        let nextReady = expectation(description: "second synthesized during first playback")
        var generated: [String] = [], played: [String] = []
        try await SpeechReadback.run(["one", "two", "three"], synthesize: { text in
            generated.append(text)
            if text == "two" { nextReady.fulfill() }
            return Data(text.utf8)
        }, play: { data in
            let text = String(decoding: data, as: UTF8.self)
            if text == "one" {
                await self.fulfillment(of: [nextReady], timeout: 2)
                XCTAssertEqual(generated, ["one", "two"])
            }
            played.append(text)
        })
        XCTAssertEqual(played, ["one", "two", "three"])
        XCTAssertEqual(generated, played)
    }
    @MainActor
    func testEmptyReadbackDoesNoWork() async throws {
        try await SpeechReadback.run([], synthesize: { _ in
            XCTFail("unexpected synthesis"); return Data()
        }, play: { _ in XCTFail("unexpected playback") })
    }
    @MainActor
    func testSynthesisFailureDoesNotSkipToLaterChunks() async {
        var played: [String] = []
        do {
            try await SpeechReadback.run(["one", "two", "three"], synthesize: { text in
                if text == "two" { throw ClientError.invalidResponse }
                return Data(text.utf8)
            }, play: { played.append(String(decoding: $0, as: UTF8.self)) })
            XCTFail("expected error")
        } catch { XCTAssertEqual(played, ["one"]) }
    }
}