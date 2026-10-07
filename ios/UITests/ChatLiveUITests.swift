import XCTest

/// Physical-device test: uses the previously provisioned Keychain, never a fixture token.
final class ChatLiveUITests: XCTestCase {
    func testSendRealMessageFromChatScreen() throws {
        let app = XCUIApplication()
        app.launchArguments = ["--uitesting"]
        app.launch()
        XCTAssertTrue(app.navigationBars["CodeTether Chat"].waitForExistence(timeout: 60))
        XCTAssertTrue(app.staticTexts["agent-mode"].waitForExistence(timeout: 30))
        app.buttons["new-chat"].tap()
        let ready = expectation(for: NSPredicate(format: "enabled == true"), evaluatedWith: app.buttons["new-chat"])
        wait(for: [ready], timeout: 30)
        let readAloud = app.switches["read-aloud-toggle"]
        if readAloud.exists && readAloud.value as? String == "0" {
            readAloud.coordinate(withNormalizedOffset: CGVector(dx: 0.8, dy: 0.5)).tap()
        }
        let input = app.descendants(matching: .any)["chat-input"].firstMatch
        XCTAssertTrue(input.waitForExistence(timeout: 30))
        input.tap()
        let marker = "IPHONE_AGENT_\(Int(Date().timeIntervalSince1970))"
        input.typeText("Use exec_command: printf \(marker). Only this command.")
        let send = app.buttons["chat-send"]
        let enabled = expectation(for: NSPredicate(format: "enabled == true"), evaluatedWith: send)
        wait(for: [enabled], timeout: 90)
        send.tap()
        let reply = app.staticTexts.matching(identifier: "assistant-message").matching(NSPredicate(format: "label CONTAINS %@", marker)).firstMatch
        XCTAssertTrue(reply.waitForExistence(timeout: 180))
        XCTAssertFalse(reply.label.isEmpty)
        XCTAssertTrue(app.staticTexts["Kokoro playback finished"].waitForExistence(timeout: 180))
        if app.buttons["Stop response"].exists { app.buttons["Stop response"].tap() }
        let sessionID = app.staticTexts["chat-session-id"].label.replacingOccurrences(of: "Session ", with: "")
        app.buttons["saved-chats"].tap()
        XCTAssertTrue(app.navigationBars["Saved chats"].waitForExistence(timeout: 20))
        let saved = app.buttons["conversation-\(sessionID)"]
        XCTAssertTrue(saved.waitForExistence(timeout: 30))
        saved.tap()
        XCTAssertTrue(reply.waitForExistence(timeout: 30))
        app.terminate()
        app.launch()
        XCTAssertTrue(reply.waitForExistence(timeout: 30))
        let screenshot = XCTAttachment(screenshot: app.screenshot())
        screenshot.name = "Physical iPhone — real chat response"
        screenshot.lifetime = .keepAlways
        add(screenshot)
    }
}