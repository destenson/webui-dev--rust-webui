
use bindgen;

use std::env;
use std::path::PathBuf;

fn main() {

    // WebUI static lib
    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-link-search=native=./");
    }
    #[cfg(target_os = "windows")]
    {
        // This is needed so that consumers of this crate can link to the static library
        let current_dir = std::env::current_dir().unwrap();
        println!("cargo:rustc-link-search=native={}", current_dir.display());
    }

    println!("cargo:rustc-link-lib=webui-2-static");

    #[cfg(target_os = "windows")]
    {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=advapi32");
        println!("cargo:rustc-link-lib=ole32");
    }

    // Specify the path to the header file of the `webui` library
    let webui_root = env::var("WEBUI_ROOT").expect("WEBUI_ROOT must be set");
    let header_path = format!("{}/include/webui.h", webui_root);
    
    // Tell cargo to invalidate the built crate whenever the header changes
    println!("cargo:rerun-if-changed={}", header_path);

    // Generate the bindings
    let bindings = bindgen::Builder::default()
        .header(header_path)
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}