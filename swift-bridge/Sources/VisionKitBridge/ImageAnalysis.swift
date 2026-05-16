import Foundation
import VisionKit

@available(macOS 13.0, *)
final class VKImageAnalysisBox: NSObject {
    let analysis: ImageAnalysis

    init(analysis: ImageAnalysis) {
        self.analysis = analysis
        super.init()
    }
}

@available(macOS 13.0, *)
func vkImageAnalysisBox(_ token: UnsafeMutableRawPointer?) throws -> VKImageAnalysisBox {
    guard let token else {
        throw VKBridgeError.invalidArgument("missing image analysis token")
    }
    return vkBorrow(token)
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
        outHasResults.pointee = box.analysis.hasResults(
            for: vkAnalyzerAnalysisTypes(from: analysisTypesRaw)
        ) ? 1 : 0
        return VK_OK
    } catch let error as VKBridgeError {
        outErrorMessage?.pointee = vkCString(error.description)
        return error.statusCode
    } catch {
        outErrorMessage?.pointee = vkCString(error.localizedDescription)
        return vkStatus(from: error)
    }
}
