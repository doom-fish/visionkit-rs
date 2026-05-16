import AppKit
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
final class VKImageAnalysisBox: NSObject {
    let analysis: ImageAnalysis

    init(analysis: ImageAnalysis) {
        self.analysis = analysis
        super.init()
    }
}

@available(macOS 13.0, *)
func vkImageAnalyzerBox(_ token: UnsafeMutableRawPointer?) throws -> VKImageAnalyzerBox {
    guard let token else {
        throw VKBridgeError.invalidArgument("missing image analyzer token")
    }
    return vkBorrow(token)
}

@available(macOS 13.0, *)
func vkImageAnalysisBox(_ token: UnsafeMutableRawPointer?) throws -> VKImageAnalysisBox {
    guard let token else {
        throw VKBridgeError.invalidArgument("missing image analysis token")
    }
    return vkBorrow(token)
}

@available(macOS 13.0, *)
func vkAnalysisTypes(from raw: UInt64) -> ImageAnalyzer.AnalysisTypes {
    ImageAnalyzer.AnalysisTypes(rawValue: UInt(raw))
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
        let json = try vkEncodeJSON(ImageAnalyzer.supportedTextRecognitionLanguages.sorted())
        outLanguagesJson.pointee = vkCString(json)
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
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "ImageAnalyzer requires macOS 13+"
            )
        }
        guard ImageAnalyzer.isSupported else {
            throw VKBridgeError.analyzerNotSupported(
                "ImageAnalyzer is not supported on this Mac"
            )
        }
        let box = try vkImageAnalyzerBox(token)
        let path = try vkRequireString(path, field: "path")
        guard FileManager.default.fileExists(atPath: path) else {
            throw VKBridgeError.invalidArgument(
                "image file does not exist at path: \(path)"
            )
        }
        let orientation = try vkImageOrientation(from: orientationRaw)
        let payload = try vkDecodeJSON(
            configurationJson,
            as: VKImageAnalyzerConfigurationPayload.self
        )
        var configuration = ImageAnalyzer.Configuration(vkAnalysisTypes(from: payload.analysisTypes))
        configuration.locales = payload.locales
        let analysis = try vk_block_on_async {
            try await box.analyzer.analyze(
                imageAt: URL(fileURLWithPath: path),
                orientation: orientation,
                configuration: configuration
            )
        }
        outAnalysisToken.pointee = vkRetain(VKImageAnalysisBox(analysis: analysis))
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_image_analysis_release")
public func vk_image_analysis_release(_ token: UnsafeMutableRawPointer?) {
    guard let token else { return }
    vkRelease(token)
}

@_cdecl("vk_image_analysis_transcript")
public func vk_image_analysis_transcript(
    _ token: UnsafeMutableRawPointer?,
    _ outTranscript: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "ImageAnalysis requires macOS 13+"
            )
        }
        let box = try vkImageAnalysisBox(token)
        outTranscript.pointee = vkCString(box.analysis.transcript)
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}

@_cdecl("vk_image_analysis_has_results")
public func vk_image_analysis_has_results(
    _ token: UnsafeMutableRawPointer?,
    _ analysisTypesRaw: UInt64,
    _ outHasResults: UnsafeMutablePointer<Int32>,
    _ outErrorMessage: UnsafeMutablePointer<UnsafeMutablePointer<CChar>?>?
) -> Int32 {
    do {
        guard #available(macOS 13.0, *) else {
            throw VKBridgeError.unavailableOnThisMacOS(
                "ImageAnalysis requires macOS 13+"
            )
        }
        let box = try vkImageAnalysisBox(token)
        outHasResults.pointee = box.analysis.hasResults(for: vkAnalysisTypes(from: analysisTypesRaw)) ? 1 : 0
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}
