import XCTest

/// Live physical iPhone/relay session check using existing Keychain credentials.
/// Never pairs a device, requests a screenshot, or sends keyboard input.
final class ScreenLiveUITests: XCTestCase {
    func testCreateStreamAndStopScreenSession() {
        let app = XCUIApplication()
        app.launchArguments = ["--uitesting"]
        app.launch()
        XCTAssertTrue(app.tabBars.buttons["Screen"].waitForExistence(timeout: 40))
        app.tabBars.buttons["Screen"].tap()
        let field = app.textFields["screen-model"]
        XCTAssertTrue(field.waitForExistence(timeout: 20))
        let previous = field.value as? String ?? ""
        if previous.isEmpty || previous == "provider/vision-model" {
            field.tap()
            field.typeText("openai-codex/gpt-5.3-codex\n")
        }
        let create = app.buttons["screen-create"]
        if !create.isHittable { app.swipeUp() }
        XCTAssertTrue(create.waitForExistence(timeout: 10))
        XCTAssertTrue(create.isEnabled)
        create.tap()
        defer {
            let stop = app.buttons["screen-stop"]
            for _ in 0..<5 where !stop.isHittable { app.swipeUp() }
            if stop.exists && stop.isHittable {
                stop.tap()
                let confirm = app.buttons["Stop session"].firstMatch
                if confirm.waitForExistence(timeout: 5) { confirm.tap() }
            }
        }
        let stream = app.staticTexts["Live analysis stream"]
        for _ in 0..<3 where !stream.isHittable { app.swipeUp() }
        XCTAssertTrue(stream.waitForExistence(timeout: 30))
        XCTAssertFalse(app.staticTexts["screen-error"].exists)
        // Pairing-code labels are neither printed nor saved as custom attachments.
        XCTAssertTrue(app.buttons["screen-stop"].exists)
    }
}