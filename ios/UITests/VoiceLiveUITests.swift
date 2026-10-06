import XCTest

final class VoiceLiveUITests: XCTestCase {
    func testKokoroSpeakerPlayback() {
        let app = XCUIApplication()
        app.launchArguments = ["--uitesting"]
        app.launch()
        let test = app.buttons["test-speaker"]
        XCTAssertTrue(test.waitForExistence(timeout: 60))
        test.tap()
        let playing = app.staticTexts["Playing Kokoro audio"]
        XCTAssertTrue(playing.waitForExistence(timeout: 90))
        let started = XCTAttachment(screenshot: app.screenshot())
        started.name = "Physical iPhone — Kokoro playback active"
        started.lifetime = .keepAlways
        add(started)
        XCTAssertTrue(app.staticTexts["Kokoro playback finished"].waitForExistence(timeout: 90))
        let finished = XCTAttachment(screenshot: app.screenshot())
        finished.name = "Physical iPhone — Kokoro playback finished"
        finished.lifetime = .keepAlways
        add(finished)
    }
}
