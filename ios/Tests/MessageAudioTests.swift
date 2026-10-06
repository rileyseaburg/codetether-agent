import XCTest
@testable import CodeTether

final class MessageAudioTests: XCTestCase {
    @MainActor
    func testPerMessagePlaybackSelectionAndStopAreImmediate() {
        let voice = VoiceOutput()
        let first = UUID(), second = UUID()
        voice.speak("First message", messageID: first)
        XCTAssertEqual(voice.activeMessageID, first)
        XCTAssertTrue(voice.speaking)
        voice.speak("Second message", messageID: second)
        XCTAssertEqual(voice.activeMessageID, second)
        voice.stop()
        XCTAssertNil(voice.activeMessageID)
        XCTAssertFalse(voice.speaking)
    }
    func testRestoredAssistantRevisionsCollapseWithoutLosingUserTurns() throws {
        let json = #"{"id":"s","messages":[{"role":"user","content":[{"type":"text","text":"Question"}]},{"role":"assistant","content":[{"type":"text","text":"Working"}]},{"role":"assistant","content":[{"type":"text","text":"Answer"}]},{"role":"user","content":[{"type":"text","text":"Next question"}]}]}"#
        let session = try JSONDecoder().decode(AgentSession.self, from: Data(json.utf8))
        XCTAssertEqual(session.transcript.map(\.content), ["Question", "Answer", "Next question"])
    }
}
