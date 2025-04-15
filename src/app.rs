use glfw_bindings::{
    GLFWwindow, glfw_cleanup, glfw_create_window, glfw_init_no_api, glfw_poll_events,
    glfw_window_should_close,
};

pub struct App {
    window: Option<*mut GLFWwindow>,
}

impl Default for App {
    fn default() -> Self {
        Self { window: None }
    }
}

impl App {
    pub fn run(self: &mut Self) {
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
    }
}

impl Drop for App {
    fn drop(&mut self) {
        self.cleanup();
    }
}
