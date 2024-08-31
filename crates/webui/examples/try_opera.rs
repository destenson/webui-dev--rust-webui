use webui::webui::{wait, Window, Size};

fn main() {
    // let size = Size { width: 480, height: 240 };
    let size = Size { width: 640, height: 480 };
    let win = Window::new_with_size(size);
    let content = "<html>Hello World!</html>";

    win.show_browser(content, webui_sys::WebUIBrowser::Opera);

    std::thread::sleep(std::time::Duration::from_secs(3));
    win.close();

    wait();
}
