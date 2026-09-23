import CoreGraphics
import Foundation
import ImageIO
import VisionKit

// ============================================================================
// Shared callback type used by all async thunks:
//   arg0 – opaque result pointer. For JSON-returning thunks this is an
//           UnsafeMutablePointer<CChar> cast to UnsafeRawPointer.
//   arg1 – status code (VK_OK on success)
//   arg2 – error C-string (nil on success)
//   arg3 – Rust context pointer passed through unchanged
// ============================================================================

public typealias VKAsyncCallback = @convention(c) (
    UnsafeRawPointer?,
    Int32,
    UnsafePointer<CChar>?,
    UnsafeMutableRawPointer
) -> Void

func vkAsyncFail(
    _ error: Error,
    _ cb: VKAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    let status: Int32
    let message: String
    if let bridgeError = error as? VKBridgeError {
        status = bridgeError.statusCode
        message = bridgeError.description
    } else {
        status = vkStatus(from: error)
        message = error.localizedDescription
    }
    message.withCString { cb(nil, status, $0, ctx) }
}

// ============================================================================
// ImageAnalyzer.analyze(_:configuration:) async throws → ImageAnalysis
// ============================================================================

/// Async thunk for `ImageAnalyzer.analyze(imageAt:orientation:configuration:)`.
///
/// Fires `cb(retained VKImageAnalysisBox ptr, VK_OK, nil, ctx)` on success,
/// or `cb(nil, status, error C-string, ctx)` on failure.
@_cdecl("vk_image_analyzer_analyze_image_async")
public func vk_image_analyzer_analyze_image_async(
    _ token: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ orientationRaw: UInt32,
    _ configurationJson: UnsafePointer<CChar>?,
    _ cb: VKAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    guard #available(macOS 13.0, *) else {
        vkAsyncFail(
            VKBridgeError.unavailableOnThisMacOS("ImageAnalyzer requires macOS 13+"),
            cb,
            ctx
        )
        return
    }
    // Copy C strings synchronously — Rust may free them as soon as we return.
    let pathStr: String? = path.map { String(cString: $0) }
    let cfgStr: String? = configurationJson.map { String(cString: $0) }

    // Validate eagerly and synchronously so that error futures resolve without
    // waiting for the analysis Task.
    guard ImageAnalyzer.isSupported else {
        vkAsyncFail(
            VKBridgeError.analyzerNotSupported("ImageAnalyzer is not supported on this Mac"),
            cb,
            ctx
        )
        return
    }
    guard let rawPath = pathStr else {
        vkAsyncFail(VKBridgeError.invalidArgument("missing path"), cb, ctx)
        return
    }
    guard FileManager.default.fileExists(atPath: rawPath) else {
        vkAsyncFail(
            VKBridgeError.invalidArgument("file does not exist at path: \(rawPath)"),
            cb,
            ctx
        )
        return
    }
    guard let cfgString = cfgStr else {
        vkAsyncFail(VKBridgeError.invalidArgument("missing configuration JSON"), cb, ctx)
        return
    }

    let box: VKImageAnalyzerBox
    let orientation: CGImagePropertyOrientation
    let configuration: ImageAnalyzer.Configuration
    do {
        box = try vkImageAnalyzerBox(token)
        orientation = try vkImageOrientation(from: orientationRaw)
        configuration = try cfgString.withCString { try vkAnalyzerConfiguration(from: $0) }
    } catch {
        vkAsyncFail(error, cb, ctx)
        return
    }

    let capturedCtx = ctx
    Task {
        do {
            let analysis = try await box.analyzer.analyze(
                imageAt: URL(fileURLWithPath: rawPath),
                orientation: orientation,
                configuration: configuration
            )
            cb(vkRetain(VKImageAnalysisBox(analysis: analysis)), VK_OK, nil, capturedCtx)
        } catch {
            vkAsyncFail(error, cb, capturedCtx)
        }
    }
}

// ============================================================================
// ImageAnalysisOverlayView.subjects async throws → [Subject]
// (macOS: subjects is on ImageAnalysisOverlayView, wrapped as LiveTextInteraction)
// ============================================================================

/// JSON payload for a single subject's bounds rectangle.
/// Uses CodingKeys to produce short `x/y/width/height` JSON keys while
/// satisfying the SwiftLint `identifier_name` rule.
private struct VKSubjectBoundsPayload: Codable {
    var posX: Double
    var posY: Double
    var posWidth: Double
    var posHeight: Double

    enum CodingKeys: String, CodingKey {
        case posX = "x"
        case posY = "y"
        case posWidth = "width"
        case posHeight = "height"
    }
}

/// Async thunk for `ImageAnalysisOverlayView.subjects` (macOS).
///
/// On macOS the subject APIs live on `ImageAnalysisOverlayView`
/// (Rust: `LiveTextInteraction`), not on `ImageAnalysis`.
///
/// Fires `cb(json_ptr, VK_OK, nil, ctx)` on success where the result pointer is a
/// JSON-encoded array of `{"x":…,"y":…,"width":…,"height":…}` objects
/// (one per subject). Fires `cb(nil, status, error C-string, ctx)` on failure.
@_cdecl("vk_live_text_overlay_subjects_async")
public func vk_live_text_overlay_subjects_async(
    _ token: UnsafeMutableRawPointer?,
    _ cb: VKAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    guard #available(macOS 13.0, *) else {
        vkAsyncFail(
            VKBridgeError.unavailableOnThisMacOS("LiveTextInteraction requires macOS 13+"),
            cb,
            ctx
        )
        return
    }
    let box: VKLiveTextInteractionBox
    do {
        box = try vkLiveTextInteractionBox(token)
    } catch {
        vkAsyncFail(error, cb, ctx)
        return
    }
    let capturedCtx = ctx
    Task { @MainActor in
        do {
            let subjects = await box.overlayView.subjects
            var payloads: [VKSubjectBoundsPayload] = []
            for subject in subjects {
                let bounds = subject.bounds
                payloads.append(VKSubjectBoundsPayload(
                    posX: Double(bounds.origin.x),
                    posY: Double(bounds.origin.y),
                    posWidth: Double(bounds.size.width),
                    posHeight: Double(bounds.size.height)
                ))
            }
            let json = try vkEncodeJSON(payloads)
            json.withCString { ptr in
                cb(UnsafeRawPointer(ptr), VK_OK, nil, capturedCtx)
            }
        } catch {
            vkAsyncFail(error, cb, capturedCtx)
        }
    }
}

// ============================================================================
// ImageAnalysisOverlayView.subject(at:) async → Subject?
// ============================================================================

/// Async thunk for `ImageAnalysisOverlayView.subject(at:)` (macOS).
///
/// Fires `cb(result_ptr, VK_OK, nil, ctx)` on success where `result_ptr` is a
/// JSON C-string that is either:
/// - `"null"` – no subject found at the given point, or
/// - `{"x":…,"y":…,"width":…,"height":…}` – the bounds of the found subject.
///
/// Fires `cb(nil, status, error C-string, ctx)` on failure.
@_cdecl("vk_live_text_overlay_subject_at_async")
public func vk_live_text_overlay_subject_at_async(
    _ token: UnsafeMutableRawPointer?,
    _ pointX: Double,
    _ pointY: Double,
    _ cb: VKAsyncCallback,
    _ ctx: UnsafeMutableRawPointer
) {
    guard #available(macOS 13.0, *) else {
        vkAsyncFail(
            VKBridgeError.unavailableOnThisMacOS("LiveTextInteraction requires macOS 13+"),
            cb,
            ctx
        )
        return
    }
    let box: VKLiveTextInteractionBox
    do {
        box = try vkLiveTextInteractionBox(token)
    } catch {
        vkAsyncFail(error, cb, ctx)
        return
    }
    let capturedCtx = ctx
    Task { @MainActor in
        do {
            let point = CGPoint(x: pointX, y: pointY)
            if let subject = await box.overlayView.subject(at: point) {
                let bounds = subject.bounds
                let payload = VKSubjectBoundsPayload(
                    posX: Double(bounds.origin.x),
                    posY: Double(bounds.origin.y),
                    posWidth: Double(bounds.size.width),
                    posHeight: Double(bounds.size.height)
                )
                let json = try vkEncodeJSON(payload)
                json.withCString { ptr in
                    cb(UnsafeRawPointer(ptr), VK_OK, nil, capturedCtx)
                }
            } else {
                "null".withCString { ptr in
                    cb(UnsafeRawPointer(ptr), VK_OK, nil, capturedCtx)
                }
            }
        } catch {
            vkAsyncFail(error, cb, capturedCtx)
        }
    }
}


/// Pump the main `RunLoop` for up to `milliseconds` ms, processing any pending
/// events (main-actor Swift Tasks, GCD blocks, etc.).  Must only be called from
/// the main thread; on non-main threads the function returns immediately.
@_cdecl("vk_pump_main_run_loop")
public func vk_pump_main_run_loop(_ milliseconds: UInt32) {
    guard Thread.isMainThread else { return }
    RunLoop.main.run(
        mode: .default,
        before: Date(timeIntervalSinceNow: Double(milliseconds) / 1_000.0)
    )
}
