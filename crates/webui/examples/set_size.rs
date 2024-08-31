use webui::webui::{wait, Window, Size};
use webui_sys::bindgen::webui_set_size;

fn main() {
    let size = Size { width: 360, height: 240 };
    // let size = Size { width: 640, height: 480 };

    // webui_set_size(win, width, height)
    let win = Window::new();
    println!("Window size: {:?}", size);
    win.set_size(size);

    let content = "<html>Hello World!</html>";
    win.show(content);

    // std::thread::sleep(std::time::Duration::from_secs(3));
    // win.close();

    wait();
}
