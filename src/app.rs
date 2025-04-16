use std::{ffi::CString, thread::sleep, time::Duration};

use glfw_bindings::{
    GLFWwindow, glfw_cleanup, glfw_create_window, glfw_get_required_instance_extensions,
    glfw_init_no_api, glfw_poll_events, glfw_window_should_close,
};
use vulkan_bindings::{
    VkDevice_T, VkInstance_T, VkPhysicalDevice_T, VkQueue_T, vk_create_instance,
    vk_create_logical_device, vk_destroy_instance, vk_get_device_queue, vk_get_physical_device,
};

pub struct App {
    window: Option<*mut GLFWwindow>,
    instance: Option<*mut VkInstance_T>,
    validation_layers: Option<Vec<CString>>,
    physical_device: Option<*mut VkPhysicalDevice_T>,
    logical_device: Option<*mut VkDevice_T>,
    graphics_queue: Option<*mut VkQueue_T>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            instance: None,
            validation_layers: None,
            physical_device: None,
            logical_device: None,
            graphics_queue: None,
        }
    }
}

impl App {
    pub fn run(self: &mut Self) {
        self.init_vulkan();
        self.init_window();
        self.main_loop();
        self.cleanup();
    }
    fn init_window(self: &mut Self) {
        glfw_init_no_api();
        match glfw_create_window(800, 600, "Vulkan") {
            Ok(result) => self.window = Some(result),
            Err(_) => panic!("Could not crate glfw window"),
        }
    }

    fn main_loop(self: &mut Self) {
        while !glfw_window_should_close(
            self.window
                .expect("Cant start main loop without initializing"),
        ) {
            glfw_poll_events();
        }
    }

    fn cleanup(self: &mut Self) {
        glfw_cleanup(
            self.window
                .expect("Cant start cleanup without initializing"),
        );
        vk_destroy_instance(self.instance.unwrap(), self.logical_device.unwrap());
    }

    fn init_vulkan(self: &mut Self) {
        self.get_validation_layers();
        let (count, extensions) = glfw_get_required_instance_extensions();

        let instance = match vk_create_instance(
            "Hello Triangle",
            count,
            extensions,
            &self.validation_layers.as_mut().unwrap(),
        ) {
            Ok(instance) => instance,
            Err(err) => {
                panic!("Could not create vulkan instance {:?}", err)
            }
        };
        self.instance = Some(instance);
        self.get_physical_device();
        self.create_logical_device();
        self.get_device_queue();
    }

    fn get_physical_device(self: &mut Self) {
        if let Some(instance) = self.instance {
            self.physical_device = match vk_get_physical_device(instance) {
                Ok(physical_device) => Some(physical_device),
                Err(err) => {
                    panic!("Could not get physical device {:?}", err)
                }
            };
        } else {
            panic!("Must get instance first!")
        }
    }

    fn create_logical_device(self: &mut Self) {
        if let Some(physical_device) = self.physical_device {
            self.logical_device = match vk_create_logical_device(
                physical_device,
                &self.validation_layers.as_mut().unwrap(),
            ) {
                Ok(logical_device) => Some(logical_device),
                Err(err) => {
                    panic!("Could not create logical device {:?}", err)
                }
            };
        } else {
            panic!("Must get physical device first!")
        }
    }

    fn get_device_queue(self: &mut Self) {
        if let Some(physical_device) = self.physical_device {
            if let Some(logical_device) = self.logical_device {
                self.graphics_queue = Some(vk_get_device_queue(physical_device, logical_device));
            } else {
                panic!("Must get logical device first!")
            }
        } else {
            panic!("Must get logical device first!")
        }
    }

    fn get_validation_layers(self: &mut Self) {
        self.validation_layers = Some(vec![CString::new("VK_LAYER_KHRONOS_validation").unwrap()])
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.cleanup();
    }
}
