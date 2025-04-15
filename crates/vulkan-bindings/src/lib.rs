#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use std::{
    ffi::CString,
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

    let instance_info = VkInstanceCreateInfo {
        sType: VkStructureType_VK_STRUCTURE_TYPE_INSTANCE_CREATE_INFO,
        pNext: null(),
        flags: 0,
        pApplicationInfo: &app_info,
        enabledLayerCount: 0,
        ppEnabledLayerNames: null(),
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
                println!(
                    "Comparing '{:?}' '{:?}'",
                    i8_array_to_string(layerProperties.layerName),
                    *layer
                );
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
