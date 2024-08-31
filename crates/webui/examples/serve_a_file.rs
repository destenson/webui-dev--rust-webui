use webui::webui::{wait, Window};

fn main() {
    let win = Window::new();

    // let path = &std::env::var("CARGO_WORKSPACE_DIR").unwrap()[2..];
    // let cwd = std::env::current_dir().unwrap();
    // let path = cwd.to_str().unwrap();
    // let path = format!("{}/crates/webui/examples/html/index.html", path);
    let path = "crates/webui/examples/html/index.html";

    println!("Path: {}", path);
    win.show(path);

    wait();
}
