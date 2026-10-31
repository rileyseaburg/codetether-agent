import XCTest

/// Mocked local UI exercises real controls with silent generated audio, not live TTS.
final class ReadbackControlsUITests: XCTestCase {
    func testPlaybackButtonsScrubbingAndCaptionScrolling() {
        let app = XCUIApplication()
        app.launchArguments = ["--uitesting", "--readback-controls-fixture"]
        app.launch()
        let play = app.buttons["readback-play-pause"]
        XCTAssertTrue(play.waitForExistence(timeout: 10))
        let position = app.staticTexts["fixture-position"]
        app.buttons["readback-forward"].tap()
        XCTAssertTrue(position.waitForExistence(timeout: 3))
        XCTAssertEqual(position.label, "15")
        app.buttons["readback-rewind"].tap()
        XCTAssertEqual(position.label, "0")
        let slider = app.sliders["readback-position"]
        slider.adjust(toNormalizedSliderPosition: 0.6)
        XCTAssertGreaterThan(Int(position.label) ?? 0, 35)
        XCTAssertEqual(play.label, "Play readback")
        play.tap()
        XCTAssertEqual(play.label, "Pause readback")
        play.tap()
        XCTAssertEqual(play.label, "Play readback")
        let captions = app.scrollViews["voice-caption-scroll"]
        XCTAssertTrue(captions.exists)
        captions.swipeUp()
        XCTAssertTrue(slider.isHittable)
        XCTAssertTrue(play.isHittable)
        let image = XCTAttachment(screenshot: app.screenshot())
        image.name = "readback-controls-and-scroll"
        image.lifetime = .keepAlways; add(image)
    }
}