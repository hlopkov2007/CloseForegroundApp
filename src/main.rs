use winsafe::prelude::*;
use winsafe::{HWND, msg};
use toy_arms::{detect_keydown, VirtualKeyCode};
use winapi::um::utilapiset::Beep;
use winapi::shared::minwindef::DWORD;


fn detect_window() -> Result<HWND, String>{
    if let Some(hwnd) = <HWND as user_Hwnd>::GetForegroundWindow(){
        return Ok(hwnd);
    }

    return Err("ERROR".to_string());
}



fn main() {
    loop {

        if detect_keydown!(VirtualKeyCode::VK_END) {
            while detect_keydown!(VirtualKeyCode::VK_END) {
                println!("key pressed");
            }
            unsafe { Beep(800 as DWORD, 350 as DWORD); }
            detect_window().unwrap().SendMessage(
                msg::wm::Close {}
            );
        }
    }
}