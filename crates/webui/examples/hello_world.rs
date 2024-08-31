use webui_sys::{self, wait, Window};

fn main() {
    let size = webui_sys::Size { width: 320, height: 240 };
    let positon = webui_sys::Position { x: 320, y: 240 };
    let win = Window::new_with_size_and_position(size, positon);
    win.show("<html>Hello World!</html>");

    wait();
}
