import XCTest
@testable import CodeTether

final class ModelSelectorRowTests: XCTestCase {
    @MainActor
    func testPickerSelectionUpdatesChatModelAndPersists() {
        let chat = ChatModel()
        chat.models = ["zai/glm-4.7", "openai-codex/gpt-5.5", "bedrock/claude-sonnet"]
        chat.selectedModel = "openai-codex/gpt-5.5"
        XCTAssertEqual(chat.selectedModel, "openai-codex/gpt-5.5")
        XCTAssertEqual(UserDefaults.standard.string(forKey: "chat.model"), "openai-codex/gpt-5.5")
        UserDefaults.standard.removeObject(forKey: "chat.model")
    }
}
