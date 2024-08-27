use webui_sys::{self, wait, Window};

fn main() {
    let win = Window::new();
    win.show("<html>Hello World!</html>");

    wait();
}
