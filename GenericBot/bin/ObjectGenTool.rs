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
use dialoguer::Input;

const VK_A: c_int = 0x41;
const VK_ESCAPE: c_int = 0x1B;

fn main() {
    let hwnd = null_mut();

    unsafe {
        RegisterHotKey(hwnd, 1, (MOD_ALT | MOD_NOREPEAT).try_into().unwrap(), VK_A as u32);
        RegisterHotKey(hwnd, 2, (MOD_ALT | MOD_NOREPEAT).try_into().unwrap(), VK_ESCAPE as u32);

        let mut msg = MSG {
            hwnd: 0 as HWND,
            message: 0 as u32,
            wParam: 0 as WPARAM,
            lParam: 0 as LPARAM,
            time: 0 as u32,
            pt: POINT { x: 0, y: 0 },
        };
        print_instructions();
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
                if msg.wParam == 1 {
                    if let Some((x, y)) = get_cursor_position() {
                        let color = get_pixel_color(x, y);
                        let data = Data {
                            position: (x, y),
                            color,
                        };
                        let file_name: String = Input::new()
                            .with_prompt("Digite o nome do objeto a ser criado")
                            .interact()
                            .unwrap();
                        write_to_yaml(&format!("{}.yaml", file_name), &data);
                        println!("Objeto '{file_name}' Criado com sucesso!");
                    } else {
                        println!("Failed to get cursor position");
                    }
                    print_instructions()
                } 
                else if msg.wParam == 2 {
                    break;
                }
            }

            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}

fn print_instructions(){
    println!("----~~~~~~~~~~~~~~~~----");
    println!("Para salvar as informações correspondentes a posição do cursor pressione Alt+A e informe o nome do objeto");
    println!("Para parar o script, pressione Alt+Esc");
}

fn write_to_yaml(file_name: &str, data: &Data) {
    let serialized_data = serde_yaml::to_string(data).expect("Failed to serialize data");
    std::fs::write(file_name, serialized_data).expect("Unable to write to file");
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

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Data {
    position: (i32, i32),
    color: Option<(u8, u8, u8)>,
}
