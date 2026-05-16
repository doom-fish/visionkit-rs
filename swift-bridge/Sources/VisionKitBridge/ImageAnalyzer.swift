import AppKit
import CoreGraphics
import CoreImage
import CoreVideo
import Foundation
import ImageIO
import VisionKit

struct VKImageAnalyzerConfigurationPayload: Codable {
    var analysisTypes: UInt64
    var locales: [String]
}

@available(macOS 13.0, *)
final class VKImageAnalyzerBox: NSObject {
    let analyzer = ImageAnalyzer()
}

@available(macOS 13.0, *)
func vkImageAnalyzerBox(_ token: UnsafeMutableRawPointer?) throws -> VKImageAnalyzerBox {
    guard let token else {
        throw VKBridgeError.invalidArgument("missing image analyzer token")
    }
    return vkBorrow(token)
}

@available(macOS 13.0, *)
func vkAnalyzerAnalysisTypes(from raw: UInt64) -> ImageAnalyzer.AnalysisTypes {
    ImageAnalyzer.AnalysisTypes(rawValue: UInt(raw))
}

@available(macOS 13.0, *)
func vkAnalyzerConfiguration(
    from cString: UnsafePointer<CChar>?
) throws -> ImageAnalyzer.Configuration {
    let payload = try vkDecodeJSON(
        cString,
        as: VKImageAnalyzerConfigurationPayload.self
    )
    var configuration = ImageAnalyzer.Configuration(
        vkAnalyzerAnalysisTypes(from: payload.analysisTypes)
    )
    configuration.locales = payload.locales
    return configuration
}

@available(macOS 13.0, *)
func vkLoadNSImage(at path: String) throws -> NSImage {
    guard let image = NSImage(contentsOfFile: path) else {
        throw VKBridgeError.invalidArgument(
            "failed to load NSImage from path: \(path)"
        )
    }
    return image
}

@available(macOS 13.0, *)
func vkLoadCGImage(at path: String) throws -> CGImage {
    let url = URL(fileURLWithPath: path)
    guard let source = CGImageSourceCreateWithURL(url as CFURL, nil),
          let image = CGImageSourceCreateImageAtIndex(source, 0, nil)
    else {
        throw VKBridgeError.invalidArgument(
            "failed to load CGImage from path: \(path)"
        )
    }
    return image
}

@available(macOS 13.0, *)
func vkLoadCIImage(at path: String) throws -> CIImage {
    let url = URL(fileURLWithPath: path)
    guard let image = CIImage(contentsOf: url) else {
        throw VKBridgeError.invalidArgument(
            "failed to load CIImage from path: \(path)"
        )
    }
    return image
}

@available(macOS 13.0, *)
func vkPixelBuffer(from cgImage: CGImage) throws -> CVPixelBuffer {
    var pixelBuffer: CVPixelBuffer?
    let attributes: CFDictionary = [
        kCVPixelBufferCGImageCompatibilityKey as String: true,
        kCVPixelBufferCGBitmapContextCompatibilityKey as String: true,
        kCVPixelBufferWidthKey as String: cgImage.width,
        kCVPixelBufferHeightKey as String: cgImage.height,
    ] as CFDictionary
    let status = CVPixelBufferCreate(
        kCFAllocatorDefault,
        cgImage.width,
        cgImage.height,
        kCVPixelFormatType_32BGRA,
        attributes,
        &pixelBuffer
    )
    guard status == kCVReturnSuccess, let pixelBuffer else {
        throw VKBridgeError.framework(
            "failed to create CVPixelBuffer (status \(status))"
        )
    }

    CVPixelBufferLockBaseAddress(pixelBuffer, [])
    defer { CVPixelBufferUnlockBaseAddress(pixelBuffer, []) }

    guard let baseAddress = CVPixelBufferGetBaseAddress(pixelBuffer) else {
        throw VKBridgeError.framework(
            "CVPixelBuffer base address was nil"
        )
    }

    let bitmapInfo = CGBitmapInfo.byteOrder32Little.rawValue |
        CGImageAlphaInfo.premultipliedFirst.rawValue
    guard let context = CGContext(
        data: baseAddress,
        width: cgImage.width,
        height: cgImage.height,
        bitsPerComponent: 8,
        bytesPerRow: CVPixelBufferGetBytesPerRow(pixelBuffer),
        space: CGColorSpaceCreateDeviceRGB(),
        bitmapInfo: bitmapInfo
    ) else {
        throw VKBridgeError.framework(
            "failed to create CGContext for CVPixelBuffer"
        )
    }

    context.draw(
        cgImage,
        in: CGRect(
            x: 0,
            y: 0,
            width: CGFloat(cgImage.width),
            height: CGFloat(cgImage.height)
        )
    )
    return pixelBuffer
}

@available(macOS 13.0, *)
func vkLoadPixelBuffer(at path: String) throws -> CVPixelBuffer {
    try vkPixelBuffer(from: vkLoadCGImage(at: path))
}

@available(macOS 13.0, *)
func vkPerformImageAnalysis(
    token: UnsafeMutableRawPointer?,
    path: UnsafePointer<CChar>?,
    orientationRaw: UInt32,
    configurationJson: UnsafePointer<CChar>?,
    outAnalysisToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    work: @escaping (
        VKImageAnalyzerBox,
        String,
        CGImagePropertyOrientation,
        ImageAnalyzer.Configuration
    ) async throws -> ImageAnalysis
) -> Int32 {
    do {
        guard ImageAnalyzer.isSupported else {
            throw VKBridgeError.analyzerNotSupported(
                "ImageAnalyzer is not supported on this Mac"
            )
        }
        let box = try vkImageAnalyzerBox(token)
        let path = try vkRequireFilePath(path, field: "path")
        let orientation = try vkImageOrientation(from: orientationRaw)
        let configuration = try vkAnalyzerConfiguration(from: configurationJson)
        let analysis = try vk_block_on_main_actor_async {
            try await work(box, path, orientation, configuration)
        }
        outAnalysisToken.pointee = vkRetain(
            VKImageAnalysisBox(analysis: analysis)
        )
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_image_analyzer_new")
public func vk_image_analyzer_new() -> UnsafeMutableRawPointer? {
    if #available(macOS 13.0, *) {
        return vkRetain(VKImageAnalyzerBox())
    }
    return nil
}

@_cdecl("vk_image_analyzer_release")
public func vk_image_analyzer_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    vkRelease(token)
}

@_cdecl("vk_image_analyzer_is_supported")
public func vk_image_analyzer_is_supported() -> Int32 {
    guard #available(macOS 13.0, *) else {
        return 0
    }
    return ImageAnalyzer.isSupported ? 1 : 0
}

@_cdecl("vk_image_analyzer_supported_text_recognition_languages_json")
public func vk_image_analyzer_supported_text_recognition_languages_json(
    _ outLanguagesJson: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "ImageAnalyzer requires macOS 13+"
            )
        }
        try vkWriteJSON(
            ImageAnalyzer.supportedTextRecognitionLanguages.sorted(),
            to: outLanguagesJson
        )
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_image_analyzer_analyze_image_at_path")
public func vk_image_analyzer_analyze_image_at_path(
    _ token: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ orientationRaw: UInt32,
    _ configurationJson: UnsafePointer<CChar>?,
    _ outAnalysisToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outErrorMessage?.pointee = vkCString(
            "ImageAnalyzer requires macOS 13+"
        )
        return VK_UNAVAILABLE_ON_THIS_MACOS
    }
    return vkPerformImageAnalysis(
        token: token,
        path: path,
        orientationRaw: orientationRaw,
        configurationJson: configurationJson,
        outAnalysisToken: outAnalysisToken,
        outErrorMessage: outErrorMessage
    ) { box, path, orientation, configuration in
        try await box.analyzer.analyze(
            imageAt: URL(fileURLWithPath: path),
            orientation: orientation,
            configuration: configuration
        )
    }
}

@_cdecl("vk_image_analyzer_analyze_ns_image_at_path")
public func vk_image_analyzer_analyze_ns_image_at_path(
    _ token: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ orientationRaw: UInt32,
    _ configurationJson: UnsafePointer<CChar>?,
    _ outAnalysisToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outErrorMessage?.pointee = vkCString(
            "ImageAnalyzer requires macOS 13+"
        )
        return VK_UNAVAILABLE_ON_THIS_MACOS
    }
    return vkPerformImageAnalysis(
        token: token,
        path: path,
        orientationRaw: orientationRaw,
        configurationJson: configurationJson,
        outAnalysisToken: outAnalysisToken,
        outErrorMessage: outErrorMessage
    ) { box, path, orientation, configuration in
        try await box.analyzer.analyze(
            try vkLoadNSImage(at: path),
            orientation: orientation,
            configuration: configuration
        )
    }
}

@_cdecl("vk_image_analyzer_analyze_cg_image_at_path")
public func vk_image_analyzer_analyze_cg_image_at_path(
    _ token: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ orientationRaw: UInt32,
    _ configurationJson: UnsafePointer<CChar>?,
    _ outAnalysisToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outErrorMessage?.pointee = vkCString(
            "ImageAnalyzer requires macOS 13+"
        )
        return VK_UNAVAILABLE_ON_THIS_MACOS
    }
    return vkPerformImageAnalysis(
        token: token,
        path: path,
        orientationRaw: orientationRaw,
        configurationJson: configurationJson,
        outAnalysisToken: outAnalysisToken,
        outErrorMessage: outErrorMessage
    ) { box, path, orientation, configuration in
        try await box.analyzer.analyze(
            try vkLoadCGImage(at: path),
            orientation: orientation,
            configuration: configuration
        )
    }
}

@_cdecl("vk_image_analyzer_analyze_ci_image_at_path")
public func vk_image_analyzer_analyze_ci_image_at_path(
    _ token: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ orientationRaw: UInt32,
    _ configurationJson: UnsafePointer<CChar>?,
    _ outAnalysisToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outErrorMessage?.pointee = vkCString(
            "ImageAnalyzer requires macOS 13+"
        )
        return VK_UNAVAILABLE_ON_THIS_MACOS
    }
    return vkPerformImageAnalysis(
        token: token,
        path: path,
        orientationRaw: orientationRaw,
        configurationJson: configurationJson,
        outAnalysisToken: outAnalysisToken,
        outErrorMessage: outErrorMessage
    ) { box, path, orientation, configuration in
        try await box.analyzer.analyze(
            try vkLoadCIImage(at: path),
            orientation: orientation,
            configuration: configuration
        )
    }
}

@_cdecl("vk_image_analyzer_analyze_pixel_buffer_at_path")
public func vk_image_analyzer_analyze_pixel_buffer_at_path(
    _ token: UnsafeMutableRawPointer?,
    _ path: UnsafePointer<CChar>?,
    _ orientationRaw: UInt32,
    _ configurationJson: UnsafePointer<CChar>?,
    _ outAnalysisToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    guard #available(macOS 13.0, *) else {
        outErrorMessage?.pointee = vkCString(
            "ImageAnalyzer requires macOS 13+"
        )
        return VK_UNAVAILABLE_ON_THIS_MACOS
    }
    return vkPerformImageAnalysis(
        token: token,
        path: path,
        orientationRaw: orientationRaw,
        configurationJson: configurationJson,
        outAnalysisToken: outAnalysisToken,
        outErrorMessage: outErrorMessage
    ) { box, path, orientation, configuration in
        try await box.analyzer.analyze(
            try vkLoadPixelBuffer(at: path),
            orientation: orientation,
            configuration: configuration
        )
    }
}
