import XCTest
@testable import CodeTether

final class ChatModelSelectionTests: XCTestCase {
    func testUserChoiceWins() {
        XCTAssertEqual(ChatModelSelection.choose(models: ["a", "openai-codex/gpt-5.5"],
                                                 saved: "a", serverDefault: nil), "a")
    }
    func testVerifiedSubscriptionModelWinsOverExpiredDefault() {
        XCTAssertEqual(ChatModelSelection.choose(models: ["bedrock/model", "openai-codex/gpt-5.5"],
                        saved: nil, serverDefault: "bedrock/model"), "openai-codex/gpt-5.5")
    }
    func testOnlyAdvertisedModelsAreChosen() {
        XCTAssertEqual(ChatModelSelection.choose(models: ["server/model"], saved: "missing",
                                                 serverDefault: "server/model"), "server/model")
        XCTAssertEqual(ChatModelSelection.choose(models: [], saved: nil, serverDefault: nil), "")
    }
}
