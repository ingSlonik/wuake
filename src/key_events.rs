extern crate user32;

use std::sync::mpsc::Sender;
use std::{thread, time};

pub struct KeyEvent {
    key: i32,
    pressed: bool,
}

impl KeyEvent {
    pub fn new(key: i32) -> KeyEvent {
        unsafe {
            KeyEvent {
                key: key,
                pressed: user32::GetAsyncKeyState(key) != 0,
            }
        }
    }
    pub fn key_down(&mut self) {
        // 30 FPS - sorry all good programmers for that
        let duration = time::Duration::from_millis(33);

        loop {
            let pressed;
            unsafe {
                pressed = user32::GetAsyncKeyState(self.key) != 0;
            }
            if self.pressed == false && pressed == true {
                self.pressed = pressed;
                break;
            } else {
                self.pressed = pressed;
            }
            thread::sleep(duration);
        }
    }
}

pub fn key_down(key: i32, tx: Sender<i32>) {
    let mut key_event = KeyEvent::new(key);

    loop {
        key_event.key_down();
        match tx.send(1) {
            Err(why) => panic!("{:?}", why),
            Ok(_) => (),
        }
    }
}

pub fn get_key_on_change() -> i32 {
    // let keys: [bool; 256] = [false; 256];
    let duration = time::Duration::from_millis(33);

    for key in 0..255 {
        unsafe {
            user32::GetAsyncKeyState(key);
        }
    }

    loop {
        thread::sleep(duration);
        for key in 0..255 {
            unsafe {
                if user32::GetAsyncKeyState(key) != 0 {
                    return key;
                }
            }
        }
    }
}
