import AVFoundation

/// One seekable readback shared by app controls and lock-screen media commands.
@MainActor
final class ReadbackTransport: NSObject, ObservableObject, AVAudioPlayerDelegate {
    @Published private(set) var active = false
    @Published var paused = false
    @Published var elapsed: TimeInterval = 0
    @Published var duration: TimeInterval = 0
    @Published var generating = false
    var timeline = ReadbackTimeline()
    let cache = ReadbackClipCache()
    var player: AVAudioPlayer?
    var index = 0
    var epoch = UUID()
    var producer: Task<Void, Never>?
    var completion: CheckedContinuation<Void, Error>?
    var ticker: Timer?
    lazy var remote = ReadbackRemote(transport: self)
    func begin() {
        stop(); active = true; paused = false; generating = true
        elapsed = 0; duration = 0; index = 0; timeline = ReadbackTimeline()
        remote.attach()
        ticker = Timer.scheduledTimer(withTimeInterval: 0.25, repeats: true) { [weak self] _ in
            Task { @MainActor in self?.refresh() }
        }
    }
    func stop() { epoch = UUID(); finish(CancellationError()) }
    func finish(_ error: Error? = nil) {
        guard active || completion != nil else { return }
        active = false; generating = false
        producer?.cancel(); producer = nil
        player?.stop(); player = nil
        ticker?.invalidate(); ticker = nil; remote.detach()
        cache.discard()
        let pending = completion; completion = nil
        if let error { pending?.resume(throwing: error) } else { pending?.resume() }
    }
}

