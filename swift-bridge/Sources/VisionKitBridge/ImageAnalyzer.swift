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
enum VKAnalysisInput {
    case url(URL)
    case nsImagePath(String)
    case cgImagePath(String)
    case ciImagePath(String)
    case pixelBufferPath(String)
    case cgImage(CGImage)
    case pixelBuffer(CVPixelBuffer)

    func analyze(
        with analyzer: ImageAnalyzer,
        orientation: CGImagePropertyOrientation,
        configuration: ImageAnalyzer.Configuration
    ) async throws -> ImageAnalysis {
        switch self {
        case let .url(url):
            return try await analyzer.analyze(
                imageAt: url,
                orientation: orientation,
                configuration: configuration
            )
        case let .nsImagePath(path):
            return try await analyzer.analyze(
                try vkLoadNSImage(at: path),
                orientation: orientation,
                configuration: configuration
            )
        case let .cgImagePath(path):
            return try await analyzer.analyze(
                try vkLoadCGImage(at: path),
                orientation: orientation,
                configuration: configuration
            )
        case let .ciImagePath(path):
            return try await analyzer.analyze(
                try vkLoadCIImage(at: path),
                orientation: orientation,
                configuration: configuration
            )
        case let .pixelBufferPath(path):
            return try await analyzer.analyze(
                try vkLoadPixelBuffer(at: path),
                orientation: orientation,
                configuration: configuration
            )
        case let .cgImage(image):
            return try await analyzer.analyze(
                image,
                orientation: orientation,
                configuration: configuration
            )
        case let .pixelBuffer(pixelBuffer):
            return try await analyzer.analyze(
                pixelBuffer,
                orientation: orientation,
                configuration: configuration
            )
        }
    }
}

func vkBorrowCGImage(_ pointer: UnsafeMutableRawPointer?) throws -> CGImage {
    guard let pointer else {
        throw VKBridgeError.invalidArgument("missing CGImage")
    }
    let object = Unmanaged<AnyObject>.fromOpaque(pointer).takeUnretainedValue()
    guard CFGetTypeID(object) == CGImage.typeID else {
        throw VKBridgeError.invalidArgument("pointer is not a CGImage")
    }
    return Unmanaged<CGImage>.fromOpaque(pointer).takeUnretainedValue()
}

func vkBorrowPixelBuffer(_ pointer: UnsafeMutableRawPointer?) throws -> CVPixelBuffer {
    guard let pointer else {
        throw VKBridgeError.invalidArgument("missing CVPixelBuffer")
    }
    let object = Unmanaged<AnyObject>.fromOpaque(pointer).takeUnretainedValue()
    guard CFGetTypeID(object) == CVPixelBufferGetTypeID() else {
        throw VKBridgeError.invalidArgument("pointer is not a CVPixelBuffer")
    }
    return Unmanaged<CVPixelBuffer>.fromOpaque(pointer).takeUnretainedValue()
}

@available(macOS 13.0, *)
func vkPerformImageAnalysis(
    token: UnsafeMutableRawPointer?,
    orientationRaw: UInt32,
    configurationJson: UnsafePointer<CChar>?,
    outAnalysisToken: UnsafeMutablePointer<UnsafeMutableRawPointer?>,
    outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?,
    input: () throws -> VKAnalysisInput
) -> Int32 {
    do {
        guard ImageAnalyzer.isSupported else {
            throw VKBridgeError.analyzerNotSupported(
                "ImageAnalyzer is not supported on this Mac"
            )
        }
        let box = try vkImageAnalyzerBox(token)
        let source = try input()
        let orientation = try vkImageOrientation(from: orientationRaw)
        let configuration = try vkAnalyzerConfiguration(from: configurationJson)
        let analysis = try vk_block_on_async(
            mainQueueGraceSeconds: 10,
            label: "image analysis"
        ) {
            try await source.analyze(
                with: box.analyzer,
                orientation: orientation,
                configuration: configuration
            )
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
        orientationRaw: orientationRaw,
        configurationJson: configurationJson,
        outAnalysisToken: outAnalysisToken,
        outErrorMessage: outErrorMessage
    ) {
        try VKAnalysisInput.url(URL(fileURLWithPath: vkRequireFilePath(path, field: "path")))
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
        orientationRaw: orientationRaw,
        configurationJson: configurationJson,
        outAnalysisToken: outAnalysisToken,
        outErrorMessage: outErrorMessage
    ) {
        try VKAnalysisInput.nsImagePath(vkRequireFilePath(path, field: "path"))
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
        orientationRaw: orientationRaw,
        configurationJson: configurationJson,
        outAnalysisToken: outAnalysisToken,
        outErrorMessage: outErrorMessage
    ) {
        try VKAnalysisInput.cgImagePath(vkRequireFilePath(path, field: "path"))
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
        orientationRaw: orientationRaw,
        configurationJson: configurationJson,
        outAnalysisToken: outAnalysisToken,
        outErrorMessage: outErrorMessage
    ) {
        try VKAnalysisInput.ciImagePath(vkRequireFilePath(path, field: "path"))
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
        orientationRaw: orientationRaw,
        configurationJson: configurationJson,
        outAnalysisToken: outAnalysisToken,
        outErrorMessage: outErrorMessage
    ) {
        try VKAnalysisInput.pixelBufferPath(vkRequireFilePath(path, field: "path"))
    }
}

@_cdecl("vk_image_analyzer_analyze_cg_image")
public func vk_image_analyzer_analyze_cg_image(
    _ token: UnsafeMutableRawPointer?,
    _ image: UnsafeMutableRawPointer?,
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
        orientationRaw: orientationRaw,
        configurationJson: configurationJson,
        outAnalysisToken: outAnalysisToken,
        outErrorMessage: outErrorMessage
    ) {
        try VKAnalysisInput.cgImage(vkBorrowCGImage(image))
    }
}

@_cdecl("vk_image_analyzer_analyze_pixel_buffer")
public func vk_image_analyzer_analyze_pixel_buffer(
    _ token: UnsafeMutableRawPointer?,
    _ pixelBuffer: UnsafeMutableRawPointer?,
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
        orientationRaw: orientationRaw,
        configurationJson: configurationJson,
        outAnalysisToken: outAnalysisToken,
        outErrorMessage: outErrorMessage
    ) {
        try VKAnalysisInput.pixelBuffer(vkBorrowPixelBuffer(pixelBuffer))
    }
}
