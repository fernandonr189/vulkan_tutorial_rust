#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(improper_ctypes)]

use std::{ffi::CString, ptr::null_mut};
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

pub fn glfw_cleanup(window: *mut GLFWwindow) {
    unsafe {
        glfwDestroyWindow(window);
        glfwTerminate();
    }
}

pub fn glfw_poll_events() {
    unsafe {
        glfwPollEvents();
    }
}

pub fn glfw_window_should_close(window: *mut GLFWwindow) -> bool {
    unsafe {
        return glfwWindowShouldClose(window) == 1;
    }
}

pub fn glfw_init_no_api() {
    unsafe {
        glfwInit();
        glfwWindowHint(GLFW_CLIENT_API as i32, GLFW_NO_API as i32);
        glfwWindowHint(GLFW_RESIZABLE as i32, GLFW_FALSE as i32);
    }
}

pub fn glfw_create_window(
    heigh: i32,
    width: i32,
    window_name: &str,
) -> Result<*mut GLFWwindow, GlfwError> {
    let window_name = CString::new(window_name).expect("CStrinf failed");
    unsafe {
        let window = glfwCreateWindow(heigh, width, window_name.as_ptr(), null_mut(), null_mut());
        if window.is_null() {
            return Err(GlfwError::WindowCreationFailed);
        }
        Ok(window)
    }
}

#[derive(Debug)]
pub enum GlfwError {
    WindowCreationFailed,
}
