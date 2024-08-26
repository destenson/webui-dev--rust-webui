use webui::webui::{wait, Event, Window};

fn main() {
    let win = Window::new();

    // Declared function
    win.bind("my_button", log_to_js);

    win.show(
        r#"
  <html>
    <script src="/webui.js"></script>
    <button id="my_button">Click me for some frontend (devtools) logs!</button>
  </html>
  "#,
    );

    wait();
}

fn log_to_js(e: Event) {
    println!("# Button clicked!");
    let window = e.get_window();
    println!("# got window handle");
    let js = window.run_js(
        r#"
// console.log('Button clicked!')
"#,
    );
    if js.error {
        println!("# Error: {}", js.data);
    } else {
        println!("# No error: {}", js.data);
    }
    println!("# Button click logged!");
}
