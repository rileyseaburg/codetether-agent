import XCTest
import AVFoundation
@testable import CodeTether

@MainActor
final class VoiceAudioRoutingTests: XCTestCase {
    func testReceiverNeedsOverrideButSpeakerAndHeadphonesDoNot() {
        XCTAssertTrue(VoiceAudioRouting.needsSpeakerOverride([.builtInReceiver]))
        XCTAssertTrue(VoiceAudioRouting.needsSpeakerOverride([]))
        XCTAssertFalse(VoiceAudioRouting.needsSpeakerOverride([.builtInSpeaker]))
        XCTAssertFalse(VoiceAudioRouting.needsSpeakerOverride([.headphones]))
        XCTAssertFalse(VoiceAudioRouting.needsSpeakerOverride([.bluetoothHFP]))
        XCTAssertFalse(VoiceAudioRouting.needsSpeakerOverride([.bluetoothA2DP]))
    }

    func testMicBufferHasOwnedBytesBeforeAsyncTranscription() throws {
        let format = try XCTUnwrap(AVAudioFormat(standardFormatWithSampleRate: 16000, channels: 1))
        let input = try XCTUnwrap(AVAudioPCMBuffer(pcmFormat: format, frameCapacity: 16))
        input.frameLength = 16
        input.floatChannelData?[0][0] = 0.5
        let copied = try XCTUnwrap(VoiceAudioBuffer.copy(input))
        input.floatChannelData?[0][0] = 0
        XCTAssertEqual(copied.floatChannelData?[0][0], 0.5)
        XCTAssertEqual(copied.frameLength, 16)
    }
}
