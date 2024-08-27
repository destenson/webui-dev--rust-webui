// extern crate bindgen;
// extern crate reqwest;

use std::result::Result;
// use std::error::Error;
use std::env;
// use std::path::Path;
use std::path::{Path, PathBuf};
use reqwest::blocking::get;
use std::fs::File;
use std::io::Write;

const WEBUI_SRC_ENV: &'static str = "WEBUI_SRC";
const WEBUI_INCLUDE_PATH_ENV: &'static str = "WEBUI_INCLUDE_PATH";

lazy_static::lazy_static! {
    static ref BINDINGS_SRC_PATH: PathBuf = PathBuf::from(env::var("CARGO_MANIFEST_DIR").expect("Failed to get CARGO_MANIFEST_DIR")).join("src").join("bindings.rs");
    static ref BINDINGS_TARGET_PATH: PathBuf = PathBuf::from(env::var("OUT_DIR").expect("Failed to get OUT_DIR")).join("bindings.rs");
    // static ref LIBRARY_PATH: PathBuf = PathBuf::from(env::var("OUT_DIR").expect("Failed to get OUT_DIR")).join("darknet");
}

#[derive(Debug)]
enum Error {
    Io(std::io::Error),
    Reqwest(reqwest::Error),
    Env(env::VarError),
    Bindgen(bindgen::BindgenError),
    Unknown(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            Error::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            Error::Env(e) => write!(f, "Env error: {}", e),
            Error::Bindgen(e) => write!(f, "Bindgen error: {}", e),
            Error::Unknown(e) => write!(f, "Unknown error: {}", e),
        }
    }
}

impl From<bindgen::BindgenError> for Error {
    fn from(e: bindgen::BindgenError) -> Self {
        Error::Bindgen(e)
    }
}

fn gen_bindings<P>(include_path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    bindgen::Builder::default()
        .header(
            include_path
                .as_ref()
                .join("webui.h")
                .to_str()
                .and_then(|s| {
                    // Tell cargo to invalidate the built crate whenever the header changes
                    println!("cargo:rerun-if-changed={}", s);
                    Some(s)
                })
                .ok_or_else(|| Error::Unknown("cannot create path to webui.h".to_string()))?
        )
        .generate()?
        // .map_err(|e| Error::Bindgen(e))?
        .write_to_file(&*BINDINGS_TARGET_PATH)
        .map_err(|e| Error::Io(e))?;
    Ok(())
}

fn is_dynamic() -> bool {
    return cfg!(feature = "dylib");
}


fn get_env_var(var: &str) -> Result<String, env::VarError> {
    println!("{}{}", "cargo:rerun-if-env-changed=", var);
    return env::var(var);
}

fn download_file(url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = get(url)?;
    let mut dest = File::create(output_path)?;
    dest.write_all(&response.bytes()?)?;
    Ok(())
}

#[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
compile_error!("Unsupported target OS");

#[cfg(all(feature = "msvc", feature = "clang"))]
compile_error!("Only one of the `msvc` and `clang` features can be enabled at a time");
#[cfg(all(feature = "msvc", feature = "gcc"))]
compile_error!("Only one of the `msvc` and `gcc` features can be enabled at a time");
#[cfg(all(feature = "gcc", feature = "clang"))]
compile_error!("Only one of the `gcc` and `clang` features can be enabled at a time");

#[cfg(all(not(feature = "msvc"), not(feature = "gcc"), not(feature = "clang")))]
compile_error!("One of the `msvc`, `gcc`, or `clang` features must be enabled");


fn main() {
    println!("cargo:rerun-if-env-changed={}", WEBUI_SRC_ENV);
    println!("cargo:rerun-if-env-changed={}", WEBUI_INCLUDE_PATH_ENV);
    println!("cargo:rerun-if-env-changed={}", BINDINGS_TARGET_PATH.display());

    if cfg!(feature = "docs-rs") {
        return;
    }
    // build from source by default
    if cfg!(feature = "runtime") {
        // build_runtime()?;
    } else {
        // build_from_source()?;
    }
    // Ok(())

    // // WebUI static lib
    // #[cfg(not(target_os = "windows"))]
    // {
    //     println!("cargo:rustc-link-search=native=./");
    // }
    // #[cfg(target_os = "windows")]
    // {
    //     // This is needed so that consumers of this crate can link to the static library
    //     let current_dir = std::env::current_dir().unwrap();
    //     println!("cargo:rustc-link-search=native={}", current_dir.display());
    // }

    if ! cfg!(feature = "runtime") {
        println!("cargo:rustc-link-lib=webui-2-static");
    }

    if !(cfg!(feature = "runtime")) && (cfg!(target_os = "windows")) {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=advapi32");
        println!("cargo:rustc-link-lib=ole32");
    }

    let out_dir = get_env_var("OUT_DIR").unwrap();
    // Specify the path to the header file of the `webui` library
    let webui_root = download_or_find_webui_root(&out_dir);
    let include_path = format!("{}/include", webui_root);

    // Generate the bindings
    gen_bindings(include_path).expect("Failed to generate bindings");
    // let bindings = bindgen::Builder::default()
    //     .header(header_path)
    //     .generate()
    //     .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");

    // panic!("Done");

}

fn download_or_find_webui_root(out_dir: &str) -> String {
    match get_env_var("WEBUI_ROOT") {
        Ok(val) => val,
        Err(_) => {
            let version = get_env_var("CARGO_PKG_VERSION").unwrap();
            let homepage = get_env_var("CARGO_PKG_HOMEPAGE").expect("homepage must be set in Cargo.toml");
            let url = format!("{}/releases/download/{}", homepage, &version);

            let os = env::consts::OS;
            let arch = match env::consts::ARCH {
                "x86_64" => "x64",
                "aarch64" => "arm64",
                arch => arch,
            };
            let compiler = if cfg!(feature = "msvc") {
                "msvc"
            } else if cfg!(feature = "gcc") {
                "gcc"
            } else if cfg!(feature = "clang") {
                "clang"
            } else {
                unreachable!();
            };
            let triplet = format!("{}-{}-{}", os, compiler, arch);

            let url = format!("{}/webui-{}.zip", url, &triplet);
            let output_dir = format!("{}/../../../../", out_dir);
            let output_path = format!("{}{}-{}", &output_dir, version, url.split("/").last().unwrap());
            if !Path::new(&output_path).exists() {
                download_file(&url, &output_path).expect(&format!("Failed to download webui {} library", version));
                panic!("Downloaded webui library from {}", url);
            }

            #[cfg(debug_assertions)]
            println!("cargo:warning=Downloaded webui library from {}", url);

            let fname = std::path::Path::new(&output_path);
            assert!(fname.exists());
            let file = std::fs::File::open(&fname).expect("Failed to open downloaded file");
            
            // Extract the zip file
            let mut zf = zip::ZipArchive::new(file).expect("Failed to open zip file");
            zf.extract(out_dir).unwrap();

            let webui_root = format!("{}/webui-{}", out_dir, triplet);
            #[cfg(debug_assertions)]
            println!("cargo:rustc-link-search=native={}/debug", &webui_root);
            #[cfg(not(debug_assertions))]
            println!("cargo:rustc-link-search=native={}", &webui_root);
            webui_root
        }
    }
}