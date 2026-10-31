import AVFoundation

/// Private, backup-excluded audio cache allows rewind without retaining WAVs in RAM.
/// Complete-until-first-unlock protection permits playback while the phone is locked.
final class ReadbackClipCache {
    private var directory: URL?
    private var bytes = 0
    private(set) var clips: [URL] = []
    func append(_ data: Data) throws -> TimeInterval {
        guard bytes + data.count <= 256 * 1024 * 1024 else { throw ReadbackTransportError.cacheLimit }
        if directory == nil {
            var url = FileManager.default.temporaryDirectory.appendingPathComponent("readback-" + UUID().uuidString)
            try FileManager.default.createDirectory(at: url, withIntermediateDirectories: true,
                attributes: [.protectionKey: FileProtectionType.completeUntilFirstUserAuthentication])
            var values = URLResourceValues(); values.isExcludedFromBackup = true
            try url.setResourceValues(values)
            directory = url
        }
        let url = directory!.appendingPathComponent("\(clips.count).wav")
        try data.write(to: url, options: [.atomic, .completeFileProtectionUntilFirstUserAuthentication])
        let audio = try AVAudioFile(forReading: url)
        let duration = Double(audio.length) / audio.processingFormat.sampleRate
        guard duration.isFinite, duration > 0 else { throw ReadbackTransportError.invalidAudio }
        clips.append(url); bytes += data.count
        return duration
    }
    func discard() {
        if let directory { try? FileManager.default.removeItem(at: directory) }
        directory = nil; clips = []; bytes = 0
    }
    deinit {
        if let directory { try? FileManager.default.removeItem(at: directory) }
    }
}
