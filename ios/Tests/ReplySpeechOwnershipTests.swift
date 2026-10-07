import XCTest
@testable import CodeTether

@MainActor
final class ReplySpeechOwnershipTests: XCTestCase {
    func testVoiceRepliesNeverReachChatReadAloudEvenAfterTabSwitch() throws {
        let chat = ChatModel()
        chat.voiceModeActive = true
        chat.prepareReplySpeech(for: .voice)
        let start = #"{"kind":"item.started","payload":{"item_id":"a","item_type":"assistant_text"}}"#
        let end = #"{"kind":"item.completed","payload":{"item_id":"a","text":"Hello"}}"#
        chat.receive(try JSONDecoder().decode(AgentFrame.Event.self, from: Data(start.utf8)))
        chat.receive(try JSONDecoder().decode(AgentFrame.Event.self, from: Data(end.utf8)))
        XCTAssertNil(chat.replyForSpeech)
        XCTAssertEqual(chat.messages.last?.content, "Hello")
        chat.voiceModeActive = false
        chat.publishReplySpeech("Final answer")
        XCTAssertNil(chat.replyForSpeech)
        XCTAssertFalse(chat.shouldReadChatReply)
    }

    func testChatReadAloudReturnsForNextChatTurnWithoutChangingPreference() {
        let chat = ChatModel()
        let preference = UserDefaults.standard.bool(forKey: "voice.enabled")
        chat.prepareReplySpeech(for: .voice)
        chat.publishReplySpeech("Voice")
        XCTAssertNil(chat.replyForSpeech)
        chat.prepareReplySpeech(for: .chat)
        chat.publishReplySpeech("Chat")
        XCTAssertEqual(chat.replyForSpeech, "Chat")
        XCTAssertTrue(chat.shouldReadChatReply)
        XCTAssertEqual(UserDefaults.standard.bool(forKey: "voice.enabled"), preference)
    }

    func testActiveVoiceScreenSuppressesChatTurnThatWasAlreadyRunning() {
        let chat = ChatModel()
        chat.prepareReplySpeech(for: .chat)
        chat.voiceModeActive = true
        chat.publishReplySpeech("Previous chat reply")
        XCTAssertFalse(chat.shouldReadChatReply)
        chat.prepareReplySpeech(for: .voice)
        XCTAssertNil(chat.replyForSpeech)
    }
}