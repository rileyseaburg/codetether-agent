import CoreML
import Foundation

/// Silero VAD v6 wrapper over the bundled CoreML model
/// (`Resources/SileroVAD.mlmodelc`). Contract per FluidInference
/// `silero-vad-unified-256ms-v6.2.1`: `audio_input` Float32 [1, 4160] =
/// 64 carried context samples **first**, then 4096 new samples
/// (256 ms @ 16 kHz); LSTM `hidden`/`cell` [1, 128]; output
/// `vad_output` speech probability plus advanced states.
@MainActor
final class SileroVAD {
    private var model: MLModel
    private var hidden: MLMultiArray
    private var cell: MLMultiArray
    private var input: MLMultiArray
    private var context = [Float](repeating: 0, count: 64)

    init(url: URL? = nil) throws {
        guard let url = url ?? Bundle.main.url(forResource: "SileroVAD", withExtension: "mlmodelc") else {
            throw VoiceVADError.modelMissing
        }
        let config = MLModelConfiguration()
        config.computeUnits = .cpuOnly
        model = try MLModel(contentsOf: url, configuration: config)
        hidden = try MLMultiArray(shape: [1, 128], dataType: .float32)
        cell = try MLMultiArray(shape: [1, 128], dataType: .float32)
        input = try MLMultiArray(shape: [1, 4160], dataType: .float32)
        reset()
    }

    /// Clears LSTM state and the carried context samples.
    func reset() {
        for arr in [hidden, cell] { for j in 0..<128 { arr[j] = 0 } }
        context = [Float](repeating: 0, count: 64)
    }

    /// Processes one 4096-sample mono 16 kHz frame; returns speech probability.
    func process(_ samples: [Float]) throws -> Float {
        guard samples.count == 4096 else { throw VoiceVADError.badFrameLength }
        for i in 0..<64 { input[i] = NSNumber(value: context[i]) }
        for (i, s) in samples.enumerated() { input[64 + i] = NSNumber(value: s) }
        let features = try MLDictionaryFeatureProvider(dictionary: [
            "audio_input": input, "hidden_state": hidden, "cell_state": cell,
        ])
        let out = try model.prediction(from: features)
        guard let p = out.featureValue(for: "vad_output")?.multiArrayValue else { throw VoiceVADError.missingOutput }
        if let h = out.featureValue(for: "new_hidden_state")?.multiArrayValue { for j in 0..<128 { hidden[j] = h[j] } }
        if let c = out.featureValue(for: "new_cell_state")?.multiArrayValue { for j in 0..<128 { cell[j] = c[j] } }
        context = Array(samples.suffix(64))
        return p[0].floatValue
    }
}

enum VoiceVADError: Error {
    case modelMissing, badFrameLength, missingOutput
}