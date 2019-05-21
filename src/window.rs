extern crate kernel32;
extern crate user32;
extern crate winapi;

use std::ffi::CString;
use std::process::Command;
use std::ptr::null_mut;
use std::{thread, time};

use self::winapi::HWND;

pub fn get_window(title: String) -> Result<HWND, String> {
    let cstring_title = CString::new(title).unwrap();
    let duration = time::Duration::from_millis(10);

    for _ in 0..1000 {
        let window_handle;
        unsafe {
            window_handle = user32::FindWindowA(null_mut(), cstring_title.as_ptr());
        }

        if window_handle != null_mut() {
            return Ok(window_handle);
        }
        thread::sleep(duration);
    }

    Err("Window is not exist :(".to_owned())
}

pub fn create_cmd() -> Result<HWND, String> {
    Command::new("powershell")
        .args(&["-Command", "start C:\\Windows\\system32\\cmd.exe"])
        .output()
        .expect("failed to execute process");

    get_window("C:\\Windows\\system32\\cmd.exe".to_owned())
}

pub fn show(main_window: HWND) {
    unsafe {
        let screen_width = user32::GetSystemMetrics(16);
        let screen_height = user32::GetSystemMetrics(17);

        let left = (screen_width as f32 * 0.2) as i32;
        let top = 0;
        let width = (screen_width as f32 * 0.6) as i32;
        let height = (screen_height as f32 * 0.5) as i32;

        user32::ShowWindow(main_window, 1);
        user32::MoveWindow(main_window, left, top, width, height, 1);
    }
}

pub fn hide(main_window: HWND) {
    unsafe {
        user32::CloseWindow(main_window);
    }
}
