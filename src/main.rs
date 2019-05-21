#![windows_subsystem = "windows"]
extern crate user32;
extern crate winapi;

mod key_events;
mod window;

use std::sync::mpsc::channel;
use std::thread;

use window::create_cmd;

fn main() {
    let mut is_shown = true;

    println!("Hello, I am Wuake single window. Your terminal in Windows.");

    let (tx, rx) = channel::<i32>();

    println!("Choose key:");
    let key = key_events::get_key_on_change();
    println!("Key is: {}", key);

    // run key_events
    let tx_ui = tx.clone();
    thread::spawn(move || {
        key_events::key_down(key, tx_ui);
    });

    let cmd_window = match create_cmd() {
        Err(why) => panic!("{:?}", why),
        Ok(win) => win,
    };

    // show for start
    window::show(cmd_window);
    
    // whole event checker
    loop {
        // close wuake when the window is close
        unsafe {
            if user32::IsWindow(cmd_window) == 0 {
                break;
            }
        }
        match rx.recv() {
            Err(why) => panic!("{:?}", why),
            Ok(mes) => match mes {
                1 => {
                    println!("TOGGLE {} {:?}", is_shown, cmd_window);
                    match is_shown {
                        true => window::hide(cmd_window),
                        false => window::show(cmd_window),
                    }
                    is_shown = !is_shown;
                }
                0 => break,
                _ => println!("Ain't special"),
            },
        }
    }
}
