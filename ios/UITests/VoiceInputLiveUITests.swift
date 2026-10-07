import XCTest

final class VoiceInputLiveUITests: XCTestCase {
    func testMicrophoneCanStartAndStopWithoutAutoSending() {
        let app = XCUIApplication()
        app.launchArguments = ["--uitesting"]
        app.launch()
        let microphone = app.buttons["voice-input"]
        XCTAssertTrue(microphone.waitForExistence(timeout: 30))
        microphone.tap()
        let springboard = XCUIApplication(bundleIdentifier: "com.apple.springboard")
        for _ in 0..<2 {
            let alert = springboard.alerts.firstMatch
            if alert.waitForExistence(timeout: 4) {
                if alert.buttons["Allow"].exists { alert.buttons["Allow"].tap() }
                else if alert.buttons["OK"].exists { alert.buttons["OK"].tap() }
            }
        }
        XCTAssertTrue(app.staticTexts["Listening — tap Stop mic, then Send"].waitForExistence(timeout: 20))
        microphone.tap()
        XCTAssertFalse(app.staticTexts["Listening — tap Stop mic, then Send"].exists)
        // Never send ambient microphone transcription during validation.
        app.buttons["new-chat"].tap()
    }
}