#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use std::{
    ffi::{CString, c_char},
    ptr::{null, null_mut},
};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn vk_destroy_instance(instance: VkInstance, device: VkDevice) {
    unsafe {
        vkDestroyInstance(instance, null());
        vkDestroyDevice(device, null());
    }
}

pub fn vk_get_device_queue(physical_device: VkPhysicalDevice, device: VkDevice) -> VkQueue {
    let indices = vk_find_queue_families(physical_device).unwrap();
    unsafe {
        let mut queue: VkQueue = std::mem::zeroed();
        vkGetDeviceQueue(device, indices, 0, &mut queue);
        queue
    }
}

pub fn vk_create_logical_device(
    device: *mut VkPhysicalDevice_T,
    validation_layers: &Vec<CString>,
) -> Result<VkDevice, VulkanError> {
    let queue_family_index = vk_find_queue_families(device);
    let queue_priority = 1.0;
    unsafe {
        let mut vulkan_device: VkDevice = std::mem::zeroed();
        let mut device_queue_create_info: VkDeviceQueueCreateInfo = std::mem::zeroed();
        device_queue_create_info.sType = VkStructureType_VK_STRUCTURE_TYPE_DEVICE_QUEUE_CREATE_INFO;
        device_queue_create_info.queueFamilyIndex = queue_family_index.unwrap();
        device_queue_create_info.pQueuePriorities = &queue_priority;
        device_queue_create_info.queueCount = 1;

        let layer_name_ptrs: Vec<*const c_char> =
            validation_layers.iter().map(|s| s.as_ptr()).collect();

        let mut device_create_info: VkDeviceCreateInfo = std::mem::zeroed();
        let mut _device_features: VkPhysicalDeviceFeatures = std::mem::zeroed();
        device_create_info.sType = VkStructureType_VK_STRUCTURE_TYPE_DEVICE_CREATE_INFO;
        device_create_info.pQueueCreateInfos = &device_queue_create_info;
        device_create_info.queueCreateInfoCount = 1;
        device_create_info.pEnabledFeatures = &_device_features;
        device_create_info.enabledExtensionCount = 0;
        device_create_info.enabledLayerCount = layer_name_ptrs.len() as u32;
        device_create_info.ppEnabledLayerNames = if layer_name_ptrs.is_empty() {
            null()
        } else {
            layer_name_ptrs.as_ptr()
        };

        match vkCreateDevice(device, &device_create_info, null(), &mut vulkan_device) {
            VkResult_VK_SUCCESS => Ok(vulkan_device),
            _ => Err(VulkanError::LogicalDeviceCreationFailed),
        }
    }
}

fn vk_make_version(major: u32, minor: u32, patch: u32) -> u32 {
    (major << 22) | (minor << 12) | patch
}

fn vk_make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    (variant << 29) | (major << 22) | (minor << 12) | patch
}

fn vk_find_queue_families(device: *mut VkPhysicalDevice_T) -> Result<u32, VulkanError> {
    let mut queue_family_count: u32 = 0;
    unsafe {
        vkGetPhysicalDeviceQueueFamilyProperties(device, &mut queue_family_count, null_mut());
        let mut queue_families: Vec<VkQueueFamilyProperties> =
            vec![std::mem::zeroed(); queue_family_count as usize];
        vkGetPhysicalDeviceQueueFamilyProperties(
            device,
            &mut queue_family_count,
            queue_families.as_mut_ptr(),
        );

        for (i, queue_family) in queue_families.iter().enumerate() {
            if queue_family.queueFlags & VkQueueFlagBits_VK_QUEUE_GRAPHICS_BIT != 0 {
                return Ok(i as u32);
            }
        }
        Err(VulkanError::NoQueueFamilyIndices)
    }
}

pub fn vk_get_physical_device(instance: VkInstance) -> Result<VkPhysicalDevice, VulkanError> {
    let mut device_count: u32 = 0;
    #[allow(unused_assignments)]
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
            if !is_device_suitable(*device) {
                index += 1;
                continue;
            }
            let device_info = get_device_info(*device);
            let score = rate_device_suitability(&device_info.properties);
            if score > highest_score {
                highest_score = score;
                best_index = index;
            }
            index += 1;
        }
        physical_device = devices[best_index];
        //let selected_device_info = get_device_info(physical_device);
        //selected_device_info.print_info();
        if !is_device_suitable(physical_device) {
            return Err(VulkanError::PhysicalDeviceCreationFailed);
        }
    }
    return Ok(physical_device);
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

#[allow(dead_code)]
impl VkDeviceInfo {
    fn print_info(self: &Self) {
        println!("Device properties: {:?}", self.properties);
        println!("Device features: {:?}", self.features);
    }
}
impl Default for VkDeviceInfo {
    fn default() -> Self {
        Self {
            properties: unsafe { std::mem::zeroed() },
            features: unsafe { std::mem::zeroed() },
        }
    }
}

fn get_device_info(device: *mut VkPhysicalDevice_T) -> VkDeviceInfo {
    unsafe {
        let mut device_info = VkDeviceInfo::default();
        vkGetPhysicalDeviceProperties(device, &mut device_info.properties);
        vkGetPhysicalDeviceFeatures(device, &mut device_info.features);
        device_info
    }
}

fn is_device_suitable(device: *mut VkPhysicalDevice_T) -> bool {
    match vk_find_queue_families(device) {
        Ok(_) => {
            return true;
            let device_info = get_device_info(device);
            device_info.properties.deviceType
                == VkPhysicalDeviceType_VK_PHYSICAL_DEVICE_TYPE_DISCRETE_GPU
                && device_info.features.geometryShader == 1
        }
        Err(_) => false,
    }
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
        let mut availableLayers: Vec<VkLayerProperties> =
            vec![std::mem::zeroed(); layerCount as usize];

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
    LogicalDeviceCreationFailed,
    PhysicalDeviceCreationFailed,
    InstanceCreationFailed,
    ValidationLayersNotAvailable,
    NoQueueFamilyIndices,
}
