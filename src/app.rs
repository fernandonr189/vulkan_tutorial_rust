use std::ffi::CString;

use glfw_bindings::{
    GLFWwindow, glfw_cleanup, glfw_create_window, glfw_get_required_instance_extensions,
    glfw_init_no_api, glfw_poll_events, glfw_window_should_close,
};
use vulkan_bindings::{VkInstance_T, vk_create_instance, vk_destroy_instance};

pub struct App {
    window: Option<*mut GLFWwindow>,
    instance: Option<*mut VkInstance_T>,
    validation_layers: Option<Vec<CString>>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            window: None,
            instance: None,
            validation_layers: None,
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
        vk_destroy_instance(self.instance.unwrap());
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
        self.instance = Some(instance)
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
