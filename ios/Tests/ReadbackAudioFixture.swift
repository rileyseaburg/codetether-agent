import Foundation

/// Silent PCM WAV fixture; no network or voice credentials are used by playback tests.
enum ReadbackAudioFixture {
    static func wav(seconds: Int = 20) -> Data {
        let size = seconds * 8000 * 2
        var data = Data("RIFF".utf8)
        func integer<T: FixedWidthInteger>(_ value: T) {
            var little = value.littleEndian
            withUnsafeBytes(of: &little) { data.append(contentsOf: $0) }
        }
        integer(UInt32(36 + size)); data.append(Data("WAVEfmt ".utf8))
        integer(UInt32(16)); integer(UInt16(1)); integer(UInt16(1))
        integer(UInt32(8000)); integer(UInt32(16000)); integer(UInt16(2)); integer(UInt16(16))
        data.append(Data("data".utf8)); integer(UInt32(size)); data.append(Data(count: size))
        return data
    }
}