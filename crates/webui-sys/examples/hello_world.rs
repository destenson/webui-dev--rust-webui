use webui_sys::{wait, Window};

fn main() {
    let win = Window::new();
    win.show("<html>Hello World!</html>");

    wait();
}
