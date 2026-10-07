import XCTest

/// Mocked local UI: choose a backend model on the actual Voice screen without audio/network.
final class VoiceModelPickerUITests: XCTestCase {
    func testVoicePickerChangesVisibleSelectedModel() {
        let app = XCUIApplication()
        app.launchArguments = ["--uitesting", "--voice-model-picker-fixture"]
        app.launch()
        let selector = app.buttons["model-selector"]
        XCTAssertTrue(selector.waitForExistence(timeout: 10))
        selector.tap()
        let second = app.buttons["model-option-provider/second"]
        XCTAssertTrue(second.waitForExistence(timeout: 5))
        second.tap()
        XCTAssertTrue(app.staticTexts["provider/second"].waitForExistence(timeout: 5))
        selector.tap()
        app.buttons["model-option-provider/first"].tap()
        XCTAssertTrue(app.staticTexts["provider/first"].waitForExistence(timeout: 5))
        let image = XCTAttachment(screenshot: app.screenshot())
        image.name = "voice-backend-model-selector"
        image.lifetime = .keepAlways
        add(image)
    }
}

