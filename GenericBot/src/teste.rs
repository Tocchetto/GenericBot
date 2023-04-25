use crate::hotkey::{ EventHandler, Hotkey, MessageWindow };
use crate::winapi::um::winuser::{ GetCursorPos, POINT };
use crate::winapi::um::wingdi::{ GetDC, ReleaseDC, COLORREF, GetPixel, RGB };

fn main() {
  // Cria uma janela para receber as mensagens de hotkey
  let wnd = MessageWindow::new().expect("Failed to create message window");

  // Registra uma hotkey "Alt + A" usando o identificador 1
  let hotkey = Hotkey::new(&wnd, 1, &["ALT", "A"]).expect("Failed to register hotkey");

  // Define o handler para a hotkey
  hotkey.register_hotkey_event_handler(
    EventHandler::new(|| {
      if let Some((x, y)) = get_cursor_position() {
        println!("Cursor position: ({}, {})", x, y);
      } else {
        println!("Failed to get cursor position");
      }
    })
  );

  // Aguarda as mensagens da janela
  wnd.run_message_loop();
}

pub fn get_cursor_position() -> Option<(i32, i32)> {
  unsafe {
    let mut point = POINT::default();
    if GetCursorPos(&mut point) == 0 {
      return None;
    }
    println!("x: {} | y: {}", point.x, point.y);
    Some((point.x, point.y))
  }
}

pub fn get_pixel_color(x: i32, y: i32) -> Option<(u8, u8, u8)> {
  unsafe {
    let hdc = GetDC(std::ptr::null_mut());
    let color = GetPixel(hdc, x, y);
    ReleaseDC(std::ptr::null_mut(), hdc);

    let r = (color & 0xff) as u8;
    let g = ((color >> 8) & 0xff) as u8;
    let b = ((color >> 16) & 0xff) as u8;

    Some((r, g, b))
  }
}