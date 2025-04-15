#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use std::{
    collections::HashMap,
    ffi::{CString, c_char},
    ptr::{null, null_mut},
};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn vk_destroy_instance(instance: VkInstance) {
    unsafe {
        vkDestroyInstance(instance, null());
    }
}

fn vk_make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}

fn vk_make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}

pub fn vk_get_physical_device(instance: VkInstance) -> VkPhysicalDevice {
    let mut device_count: u32 = 0;
    let mut physical_device: VkPhysicalDevice = null_mut();
    unsafe {
        vkEnumeratePhysicalDevices(instance, &mut device_count, null_mut());
        if device_count == 0 {
            panic!("No vulkan compatible gpus available");
        }
        let mut devices: Vec<VkPhysicalDevice> = vec![null_mut(); device_count as usize];
        vkEnumeratePhysicalDevices(instance, &mut device_count, devices.as_mut_ptr());

        let mut best_index = 0;
        let mut index = 0;
        let mut highest_score = 0;
        for device in devices.iter() {
            let device_info = get_device_info(*device);
            if !is_device_suitable(&device_info) {
                index += 1;
                continue;
            }
            let score = rate_device_suitability(&device_info.properties);
            if score > highest_score {
                highest_score = score;
                best_index = index;
            }
            index += 1;
        }

        physical_device = devices[best_index];

        if physical_device.is_null() {
            panic!("Failed to find a suitable vulkan device");
        }
    }
    physical_device
}

fn rate_device_suitability(device_properties: &VkPhysicalDeviceProperties) -> u32 {
    let mut score = 0;
    if device_properties.deviceType == VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU {
        score += 1000;
    }
    score += device_properties.limits.maxImageDimension2D;
    score
}

struct VkDeviceInfo {
    pub properties: VkPhysicalDeviceProperties,
    pub features: VkPhysicalDeviceFeatures,
}

fn get_device_info(device: *mut VkPhysicalDevice_T) -> VkDeviceInfo {
    unsafe {
        let mut device_properties = VkPhysicalDeviceProperties {
            apiVersion: 0,
            driverVersion: 0,
            vendorID: 0,
            deviceID: 0,
            deviceType: 0,
            deviceName: [0; 256],
            pipelineCacheUUID: [0; 16],
            limits: VkPhysicalDeviceLimits {
                maxImageDimension1D: 0,
                maxImageDimension2D: 0,
                maxImageDimension3D: 0,
                maxImageDimensionCube: 0,
                maxImageArrayLayers: 0,
                maxTexelBufferElements: 0,
                maxUniformBufferRange: 0,
                maxStorageBufferRange: 0,
                maxPushConstantsSize: 0,
                maxMemoryAllocationCount: 0,
                maxSamplerAllocationCount: 0,
                bufferImageGranularity: 0,
                sparseAddressSpaceSize: 0,
                maxBoundDescriptorSets: 0,
                maxPerStageDescriptorSamplers: 0,
                maxPerStageDescriptorUniformBuffers: 0,
                maxPerStageDescriptorStorageBuffers: 0,
                maxPerStageDescriptorSampledImages: 0,
                maxPerStageDescriptorStorageImages: 0,
                maxPerStageDescriptorInputAttachments: 0,
                maxPerStageResources: 0,
                maxDescriptorSetSamplers: 0,
                maxDescriptorSetUniformBuffers: 0,
                maxDescriptorSetUniformBuffersDynamic: 0,
                maxDescriptorSetStorageBuffers: 0,
                maxDescriptorSetStorageBuffersDynamic: 0,
                maxDescriptorSetSampledImages: 0,
                maxDescriptorSetStorageImages: 0,
                maxDescriptorSetInputAttachments: 0,
                maxVertexInputAttributes: 0,
                maxVertexInputBindings: 0,
                maxVertexInputAttributeOffset: 0,
                maxVertexInputBindingStride: 0,
                maxVertexOutputComponents: 0,
                maxTessellationGenerationLevel: 0,
                maxTessellationPatchSize: 0,
                maxTessellationControlPerVertexInputComponents: 0,
                maxTessellationControlPerVertexOutputComponents: 0,
                maxTessellationControlPerPatchOutputComponents: 0,
                maxTessellationControlTotalOutputComponents: 0,
                maxTessellationEvaluationInputComponents: 0,
                maxTessellationEvaluationOutputComponents: 0,
                maxGeometryShaderInvocations: 0,
                maxGeometryInputComponents: 0,
                maxGeometryOutputComponents: 0,
                maxGeometryOutputVertices: 0,
                maxGeometryTotalOutputComponents: 0,
                maxFragmentInputComponents: 0,
                maxFragmentOutputAttachments: 0,
                maxFragmentDualSrcAttachments: 0,
                maxFragmentCombinedOutputResources: 0,
                maxComputeSharedMemorySize: 0,
                maxComputeWorkGroupCount: [0; 3],
                maxComputeWorkGroupInvocations: 0,
                maxComputeWorkGroupSize: [0; 3],
                subPixelPrecisionBits: 0,
                subTexelPrecisionBits: 0,
                mipmapPrecisionBits: 0,
                maxDrawIndexedIndexValue: 0,
                maxDrawIndirectCount: 0,
                maxSamplerLodBias: 0.0,
                maxSamplerAnisotropy: 0.0,
                maxViewports: 0,
                maxViewportDimensions: [0; 2],
                viewportBoundsRange: [0.0; 2],
                viewportSubPixelBits: 0,
                minMemoryMapAlignment: 0,
                minTexelBufferOffsetAlignment: 0,
                minUniformBufferOffsetAlignment: 0,
                minStorageBufferOffsetAlignment: 0,
                minTexelOffset: 0,
                maxTexelOffset: 0,
                minTexelGatherOffset: 0,
                maxTexelGatherOffset: 0,
                minInterpolationOffset: 0.0,
                maxInterpolationOffset: 0.0,
                subPixelInterpolationOffsetBits: 0,
                maxFramebufferWidth: 0,
                maxFramebufferHeight: 0,
                maxFramebufferLayers: 0,
                framebufferColorSampleCounts: 0,
                framebufferDepthSampleCounts: 0,
                framebufferStencilSampleCounts: 0,
                framebufferNoAttachmentsSampleCounts: 0,
                maxColorAttachments: 0,
                sampledImageColorSampleCounts: 0,
                sampledImageIntegerSampleCounts: 0,
                sampledImageDepthSampleCounts: 0,
                sampledImageStencilSampleCounts: 0,
                storageImageSampleCounts: 0,
                maxSampleMaskWords: 0,
                timestampComputeAndGraphics: 0,
                timestampPeriod: 0.0,
                maxClipDistances: 0,
                maxCullDistances: 0,
                maxCombinedClipAndCullDistances: 0,
                discreteQueuePriorities: 0,
                pointSizeRange: [0.0; 2],
                lineWidthRange: [0.0; 2],
                pointSizeGranularity: 0.0,
                lineWidthGranularity: 0.0,
                strictLines: 0,
                standardSampleLocations: 0,
                optimalBufferCopyOffsetAlignment: 0,
                optimalBufferCopyRowPitchAlignment: 0,
                nonCoherentAtomSize: 0,
            },
            sparseProperties: VkPhysicalDeviceSparseProperties {
                residencyStandard2DBlockShape: 0,
                residencyStandard2DMultisampleBlockShape: 0,
                residencyStandard3DBlockShape: 0,
                residencyAlignedMipSize: 0,
                residencyNonResidentStrict: 0,
            },
        };
        vkGetPhysicalDeviceProperties(device, &mut device_properties);

        let mut device_features = VkPhysicalDeviceFeatures {
            robustBufferAccess: 0,
            fullDrawIndexUint32: 0,
            imageCubeArray: 0,
            independentBlend: 0,
            geometryShader: 0,
            tessellationShader: 0,
            sampleRateShading: 0,
            dualSrcBlend: 0,
            logicOp: 0,
            multiDrawIndirect: 0,
            drawIndirectFirstInstance: 0,
            depthClamp: 0,
            depthBiasClamp: 0,
            fillModeNonSolid: 0,
            depthBounds: 0,
            wideLines: 0,
            largePoints: 0,
            alphaToOne: 0,
            multiViewport: 0,
            samplerAnisotropy: 0,
            textureCompressionETC2: 0,
            textureCompressionASTC_LDR: 0,
            textureCompressionBC: 0,
            occlusionQueryPrecise: 0,
            pipelineStatisticsQuery: 0,
            vertexPipelineStoresAndAtomics: 0,
            fragmentStoresAndAtomics: 0,
            shaderTessellationAndGeometryPointSize: 0,
            shaderImageGatherExtended: 0,
            shaderStorageImageExtendedFormats: 0,
            shaderStorageImageMultisample: 0,
            shaderStorageImageReadWithoutFormat: 0,
            shaderStorageImageWriteWithoutFormat: 0,
            shaderUniformBufferArrayDynamicIndexing: 0,
            shaderSampledImageArrayDynamicIndexing: 0,
            shaderStorageBufferArrayDynamicIndexing: 0,
            shaderStorageImageArrayDynamicIndexing: 0,
            shaderClipDistance: 0,
            shaderCullDistance: 0,
            shaderFloat64: 0,
            shaderInt64: 0,
            shaderInt16: 0,
            shaderResourceResidency: 0,
            shaderResourceMinLod: 0,
            sparseBinding: 0,
            sparseResidencyBuffer: 0,
            sparseResidencyImage2D: 0,
            sparseResidencyImage3D: 0,
            sparseResidency2Samples: 0,
            sparseResidency4Samples: 0,
            sparseResidency8Samples: 0,
            sparseResidency16Samples: 0,
            sparseResidencyAliased: 0,
            variableMultisampleRate: 0,
            inheritedQueries: 0,
        };

        vkGetPhysicalDeviceFeatures(device, &mut device_features);

        VkDeviceInfo {
            properties: device_properties,
            features: device_features,
        }
    }
}

fn is_device_suitable(device_info: &VkDeviceInfo) -> bool {
    device_info.properties.deviceType == VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU
        && device_info.features.geometryShader == 1
}

pub fn vk_create_instance(
    app_name: &str,
    extension_count: u32,
    extensions: *const *const i8,
    validation_layers: &Vec<CString>,
) -> Result<VkInstance, VulkanError> {
    if let Err(err) = vk_check_validation_layer_support(validation_layers) {
        return Err(err);
    }

    let mut instance: VkInstance = std::ptr::null_mut();
    let c_string_app_name = std::ffi::CString::new(app_name).unwrap();
    let c_string_engine_name = std::ffi::CString::new("No Engine").unwrap();
    let app_info = VkApplicationInfo {
        sType: VkStructureType_VK_STRUCTURE_TYPE_APPLICATION_INFO,
        pNext: null(),
        pApplicationName: c_string_app_name.as_ptr(),
        applicationVersion: vk_make_version(1, 0, 0),
        pEngineName: c_string_engine_name.as_ptr(),
        engineVersion: vk_make_version(1, 0, 0),
        apiVersion: vk_make_api_version(0, 1, 0, 0),
    };

    let layer_name_ptrs: Vec<*const c_char> =
        validation_layers.iter().map(|s| s.as_ptr()).collect();

    let instance_info = VkInstanceCreateInfo {
        sType: VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: null(),
        flags: 0,
        pApplicationInfo: &app_info,
        enabledLayerCount: layer_name_ptrs.len() as u32,
        ppEnabledLayerNames: if layer_name_ptrs.is_empty() {
            null()
        } else {
            layer_name_ptrs.as_ptr()
        },
        enabledExtensionCount: extension_count,
        ppEnabledExtensionNames: extensions,
    };

    let result = unsafe { vkCreateInstance(&instance_info, null(), &mut instance) };
    if result != VkResult_VK_SUCCESS {
        Err(VulkanError::InstanceCreationFailed)
    } else {
        Ok(instance)
    }
}

pub fn vk_check_validation_layer_support(
    validationLayers: &Vec<CString>,
) -> Result<(), VulkanError> {
    let mut layerCount: u32 = 0;
    unsafe {
        vkEnumerateInstanceLayerProperties(&mut layerCount, null_mut());
        let mut availableLayers: Vec<VkLayerProperties> = vec![
            VkLayerProperties {
                layerName: [0; 256],
                specVersion: 0,
                implementationVersion: 0,
                description: [0; 256],
            };
            layerCount as usize
        ];

        vkEnumerateInstanceLayerProperties(&mut layerCount, availableLayers.as_mut_ptr());
        for layer in validationLayers {
            let mut layer_found = false;

            for layerProperties in &availableLayers {
                if i8_array_to_string(layerProperties.layerName) == *layer {
                    layer_found = true;
                    break;
                }
            }

            if !layer_found {
                return Err(VulkanError::ValidationLayersNotAvailable);
            }
        }
        Ok(())
    }
}

fn i8_array_to_string(buf: [i8; 256]) -> CString {
    let bytes: &[u8] = unsafe { std::slice::from_raw_parts(buf.as_ptr() as *const u8, buf.len()) };
    let nul_terminated = bytes.split(|&b| b == 0).next().unwrap_or(&[]);
    let string = String::from_utf8_lossy(nul_terminated).to_string();
    CString::new(string).unwrap()
}

#[derive(Debug)]
pub enum VulkanError {
    InstanceCreationFailed,
    ValidationLayersNotAvailable,
}
