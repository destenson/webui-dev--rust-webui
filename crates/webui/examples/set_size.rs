use webui::webui::{wait, Window, Size};

fn main() {
    let win = Window::new();
    win.show("<html>Hello World!</html>");
    let size = win.get_size();
    println!("Window size: {:?}", size);
    win.set_size(size);

    wait();
}
