import XCTest

final class MessageSpeakerLiveUITests: XCTestCase {
    func testMessageSpeakerPlaysStopsAndReplaysKokoro() {
        let app = XCUIApplication()
        app.launchArguments = ["--uitesting"]
        app.launch()
        let speaker = app.buttons.matching(identifier: "message-speaker-assistant").firstMatch
        XCTAssertTrue(speaker.waitForExistence(timeout: 60))
        let automatic = app.switches["read-aloud-toggle"]
        if automatic.value as? String == "1" {
            automatic.coordinate(withNormalizedOffset: CGVector(dx: 0.8, dy: 0.5)).tap()
        }
        speaker.tap()
        XCTAssertTrue(app.staticTexts["Playing Kokoro audio"].waitForExistence(timeout: 90))
        XCTAssertEqual(speaker.label, "Stop reading this message")
        speaker.tap()
        XCTAssertTrue(app.staticTexts["Playback stopped"].waitForExistence(timeout: 10))
        speaker.tap()
        XCTAssertTrue(app.staticTexts["Kokoro playback finished"].waitForExistence(timeout: 90))
        let evidence = XCTAttachment(screenshot: app.screenshot())
        evidence.name = "Per-message Kokoro controls"
        evidence.lifetime = .keepAlways
        add(evidence)
    }
}
