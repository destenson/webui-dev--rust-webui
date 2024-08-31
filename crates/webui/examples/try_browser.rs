use webui::webui::{wait, Window, Size};

fn main() {

    // get the browser type from the first argument
    let args: Vec<String> = std::env::args().collect();
    let browser = if args.len() > 1 {
        args[1].as_str()
    } else {
        "Chrome"
    };

    // let size = Size { width: 480, height: 240 };
    let size = Size { width: 640, height: 480 };

    // webui_set_size(win, width, height)
    let win = Window::new_with_size(size);
    // println!("Window size: {:?}", size);
    // win.set_size(size);

    let content = format!("<html>Hello World! (using {})</html>", browser);

    let browser = match browser.to_uppercase().as_str() {
        "ANY" => webui_sys::WebUIBrowser::AnyBrowser,
        "CHROME" => webui_sys::WebUIBrowser::Chrome,
        "FIREFOX" => webui_sys::WebUIBrowser::Firefox,
        "EDGE" => webui_sys::WebUIBrowser::Edge,
        "SAFARI" => webui_sys::WebUIBrowser::Safari,
        "CHROMIUM" => webui_sys::WebUIBrowser::Chromium,
        "OPERA" => webui_sys::WebUIBrowser::Opera,
        "BRAVE" => webui_sys::WebUIBrowser::Brave,
        "VIVALDI" => webui_sys::WebUIBrowser::Vivaldi,
        "EPIC" => webui_sys::WebUIBrowser::Epic,
        "YANDEX" => webui_sys::WebUIBrowser::Yandex,
        "CHROMIUMBASED" | "CHROMIUM-BASED" | "CHROMIUM_BASED" => webui_sys::WebUIBrowser::ChromiumBased,
        _ => webui_sys::WebUIBrowser::NoBrowser,
    };

    win.show_browser(content, browser);

    std::thread::sleep(std::time::Duration::from_secs(3));
    win.close();

    wait();
}
