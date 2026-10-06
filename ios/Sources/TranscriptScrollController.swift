import Combine
import UIKit

/// Follows actual layout growth, including streaming text and asynchronously sized images.
@MainActor
final class TranscriptScrollController: NSObject, ObservableObject {
    @Published var following = true
    weak var scrollView: UIScrollView?
    var scheduled = false
    var readerUpdateScheduled = false
    private var observations: [NSKeyValueObservation] = []

    func connect(_ view: UIScrollView) {
        guard scrollView !== view else { return }
        disconnect()
        scrollView = view
        observations = [
            view.observe(\.contentSize, options: [.initial, .new]) { [weak self] _, _ in
                self?.scheduleScroll()
            },
            view.observe(\.bounds, options: [.old, .new]) { [weak self] _, change in
                if change.oldValue?.size != change.newValue?.size { self?.scheduleScroll() }
            },
            view.observe(\.frame, options: [.old, .new]) { [weak self] _, change in
                if change.oldValue?.size != change.newValue?.size { self?.scheduleScroll() }
            },
            view.observe(\.contentOffset) { [weak self] view, _ in
                guard view.isTracking || view.isDecelerating else { return }
                self?.scheduleReaderUpdate(view)
            }
        ]
        view.panGestureRecognizer.addTarget(self, action: #selector(handlePan))
    }

    func disconnect() {
        scrollView?.panGestureRecognizer.removeTarget(self, action: #selector(handlePan))
        observations.removeAll()
        scrollView = nil
    }
}