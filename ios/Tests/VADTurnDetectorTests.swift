import Testing
@testable import CodeTether

@Test func detectorFiresStartThenEnd() {
    var detector = VADTurnDetector()
    #expect(detector.feed(0.9) == [.speechStart])   // 256ms ≥ minSpeechMs
    #expect(detector.feed(0.2).isEmpty)             // silence run 256ms
    #expect(detector.feed(0.2) == [.speechEnd])     // silence run 512ms ≥ 480
}

@Test func detectorIgnoresBriefNoiseWithLongerMinSpeech() {
    var detector = VADTurnDetector(minSpeechMs: 512)
    #expect(detector.feed(0.99).isEmpty)            // one frame < 512ms
    #expect(detector.feed(0.1).isEmpty)             // run reset
    #expect(detector.feed(0.99).isEmpty)
    #expect(detector.feed(0.99) == [.speechStart])  // two consecutive frames
}

@Test func detectorResetsBetweenTurns() {
    var detector = VADTurnDetector()
    #expect(detector.feed(0.9) == [.speechStart])
    detector.reset()
    #expect(detector.feed(0.1).isEmpty)             // not inSpeech after reset
    #expect(detector.feed(0.9) == [.speechStart])
}
