use std::convert::TryInto;
use std::ptr::null_mut;
use winapi::shared::minwindef::{LPARAM, WPARAM};
use winapi::shared::windef::HWND;
use winapi::um::winuser::{
    GetMessageW, RegisterHotKey, TranslateMessage, DispatchMessageW,
    PeekMessageW, TranslateAcceleratorW, MSG, PM_REMOVE, WM_HOTKEY, MOD_ALT, MOD_NOREPEAT,
    GetCursorPos, GetDC, ReleaseDC,
};
use winapi::um::wingdi::GetPixel;
use winapi::shared::windef::POINT;
use winapi::ctypes::c_int;

// Adicione estas constantes porque elas não estão presentes na versão 0.3.9 do winapi
const VK_A: c_int = 0x41;

fn main() {
    let hwnd = null_mut();

    unsafe {
        RegisterHotKey(hwnd, 1, (MOD_ALT | MOD_NOREPEAT).try_into().unwrap(), VK_A as u32[[]]);

        let mut msg = MSG {
            hwnd: 0 as HWND,
            message: 0 as u32,
            wParam: 0 as WPARAM,
            lParam: 0 as LPARAM,
            time: 0 as u32,
            pt: POINT { x: 0, y: 0 },
        };

        loop {
            let ret = GetMessageW(&mut msg, hwnd, 0, 0);
            if ret == 0 {
                break;
            }
            if ret == -1 {
                eprintln!("Error in GetMessageW");
                break;
            }

            if msg.message == WM_HOTKEY {
                if let Some((x, y)) = get_cursor_position() {
                    println!("Cursor position: ({}, {})", x, y);
                } else {
                    println!("Failed to get cursor position");
                }
            }

            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

pub fn get_cursor_position() -> Option<(i32, i32)> {
    unsafe {
        let mut point = POINT { x: 0, y: 0 };
        if GetCursorPos(&mut point) == 0 {
            return None;
        }
        Some((point.x, point.y))
    }
}

pub fn get_pixel_color(x: i32, y: i32) -> Option<(u8, u8, u8)> {
    unsafe {
        let hdc = GetDC(null_mut());
        let color = GetPixel(hdc, x, y);
        ReleaseDC(null_mut(), hdc);

        let r = (color & 0xff) as u8;
        let g = ((color >> 8) & 0xff) as u8;
        let b = ((color >> 16) & 0xff) as u8;

        Some((r, g, b))
    }
}
