import AVFoundation

/// Audio-engine tap buffers may be reused; own the bytes before hopping to MainActor.
enum VoiceAudioBuffer {
    static func copy(_ source: AVAudioPCMBuffer) -> AVAudioPCMBuffer? {
        guard let result = AVAudioPCMBuffer(pcmFormat: source.format, frameCapacity: source.frameLength) else { return nil }
        result.frameLength = source.frameLength
        let inputs = UnsafeMutableAudioBufferListPointer(source.mutableAudioBufferList)
        let outputs = UnsafeMutableAudioBufferListPointer(result.mutableAudioBufferList)
        for (input, output) in zip(inputs, outputs) {
            guard let from = input.mData, let to = output.mData,
                  input.mDataByteSize <= output.mDataByteSize else { return nil }
            memcpy(to, from, Int(input.mDataByteSize))
        }
        return result
    }
}