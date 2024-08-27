#![allow(dead_code)]

// extern crate bindgen;
// extern crate reqwest;

use core::panic;
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
    // Reqwest(reqwest::Error),
    // Env(env::VarError),
    Bindgen(bindgen::BindgenError),
    Unknown(String),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::Io(e) => write!(f, "IO error: {}", e),
            // Error::Reqwest(e) => write!(f, "Reqwest error: {}", e),
            // Error::Env(e) => write!(f, "Env error: {}", e),
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

// #[cfg(feature = "buildtime-bindgen")]
// compile_error!("buildtime-bindgen feature is not supported yet");

#[cfg(feature = "cplusplus")]
compile_error!("cplusplus feature is not supported yet");

#[cfg(feature = "runtime")]
compile_error!("runtime feature is not supported yet");

#[cfg(feature = "dylib")]
compile_error!("dylib feature is not supported yet");

#[cfg(feature = "clang")]
compile_error!("clang feature is not supported yet");

#[cfg(feature = "gcc")]
compile_error!("gcc feature is not supported yet");

// #[cfg(feature = "msvc")]
// compile_error!("msvc feature is not supported yet");

#[cfg(feature = "src")]
compile_error!("src feature is not supported yet");

#[cfg(feature = "docs-rs")]
compile_error!("docs-rs feature is not supported yet");

#[cfg(feature = "buildtime-bindgen")]
fn gen_bindings<P>(include_path: P) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    #[cfg(not(feature = "cplusplus"))] let filename = "webui.h";

    #[cfg(feature = "cplusplus")] let filename = "webui.hpp";

    bindgen::Builder::default()
        .header(
            include_path
                .as_ref()
                .join(filename)
                .to_str()
                .and_then(|s| {
                    // Tell cargo to invalidate the built crate whenever the header changes
                    println!("cargo:rerun-if-changed={}", s);
                    Some(s)
                })
                .ok_or_else(|| Error::Unknown(format!("cannot create path to {}", filename)))?
        )
        .allowlist_var(r#"(\w*webui\w*)"#)
        .allowlist_type(r#"(\w*webui\w*)"#)
        .allowlist_function(r#"(\w*webui\w*)"#)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()?
        // .map_err(|e| Error::Bindgen(e))?
        .write_to_file(&*BINDINGS_TARGET_PATH)
        .expect("Unable to generate bindings");

    println!("cargo:rerun-if-env-changed={}", BINDINGS_TARGET_PATH.display());
    Ok(())
    
    // let bindings = bindgen::Builder::default()
    //     .header(header_path)
    //     .generate()
    //     .expect("Unable to generate bindings");

    // // Write the bindings to the $OUT_DIR/bindings.rs file
    // let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    // bindings
    //     .write_to_file(out_path.join("bindings.rs"))
    //     .expect("Couldn't write bindings!");

}

// fn is_dynamic() -> bool {
//     return cfg!(feature = "dylib");
// }


fn get_env_var(var: &str) -> Result<String, env::VarError> {
    println!("{}{}", "cargo:rerun-if-env-changed=", var);
    env::var(var)
}

fn download_file(url: &str, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(output_path).exists() {
        println!("cargo:warning=Already downloaded webui library at {}", output_path);
        return Ok(());
    }
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
    if cfg!(feature = "buildtime-bindgen") {
        println!("cargo:warning=Using buildtime-bindgen feature");
    }
    if cfg!(feature = "msvc") {
        println!("cargo:warning=Using msvc feature");
    }

    println!("cargo:rerun-if-env-changed={}", WEBUI_SRC_ENV);
    println!("cargo:rerun-if-env-changed={}", WEBUI_INCLUDE_PATH_ENV);
    
    if cfg!(feature = "docs-rs") {
        println!("cargo:warning=Using docs-rs feature");
        return;
    }
    // build from source by default
    // if cfg!(feature = "runtime") {
    //     println!("cargo:warning=Using runtime feature");
    //     // build_runtime()?;
    // } else {
    //     println!("cargo:warning=Not using runtime feature");
    //     // build_from_source()?;
    // }
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

    if cfg!(not(feature = "runtime")) {
        println!("cargo:rustc-link-lib=webui-2-static");
        println!("cargo:warning=Not using runtime feature");
    } else {
        println!("cargo:warning=Using runtime feature");
    }

    #[cfg(windows)]
    if (cfg!(not(feature = "runtime"))) && (cfg!(target_os = "windows")) {
        println!("cargo:rustc-link-lib=user32");
        println!("cargo:rustc-link-lib=shell32");
        println!("cargo:rustc-link-lib=advapi32");
        println!("cargo:rustc-link-lib=ole32");
        // if cfg!(feature = "msvc") {
        //     println!("cargo:warning=Using msvc feature");
        //     // if debug use debug version
        //     #[cfg(debug_assertions)]
        //     println!("cargo:rustc-link-lib=msvcrtd");
        //     #[cfg(not(debug_assertions))]
        //     println!("cargo:rustc-link-lib=msvcrt");
        // }
    } else {
        println!("cargo:warning=Using runtime feature");
    }

    let out_dir = get_env_var("OUT_DIR").unwrap();
    // Specify the path to the header file of the `webui` library
    let webui_root = download_or_find_webui_root(&out_dir);
    #[cfg(debug_assertions)]
    println!("cargo:rustc-link-search=native={}/debug", &webui_root);
    #[cfg(not(debug_assertions))]
    println!("cargo:rustc-link-search=native={}", &webui_root);

    // Generate the bindings
    #[cfg(feature = "buildtime-bindgen")]
    if cfg!(feature = "buildtime-bindgen") {
        let include_path = format!("{}/include", webui_root);
        gen_bindings(include_path).expect("Failed to generate bindings");
    }

    #[cfg(not(feature = "cplusplus"))]
    #[cfg(windows)]
    if cfg!(target_os = "windows") {
        println!("cargo:rustc-link-lib=libvcruntime");
        println!("cargo:rustc-link-lib=libcpmt");
        // println!("cargo:rustc-link-lib=libmsvcrt");
    }
        
    #[cfg(feature = "cplusplus")]
    {
        // on linux link with libstdc++ or libc++ depending on the compiler
        if cfg!(target_os = "linux") {
            if cfg!(feature = "gcc") {
                println!("cargo:rustc-link-lib=stdc++");
            } else if cfg!(feature = "clang") {
                println!("cargo:rustc-link-lib=c++");
            }
        }
        // on windows link with msvcrt (static runtime)
        #[cfg(windows)]
        if cfg!(target_os = "windows") {
            println!("cargo:rustc-link-lib=libucrt");
            println!("cargo:rustc-link-lib=libvcruntime");
            println!("cargo:rustc-link-lib=libmsvcrt");
            // println!("cargo:rustc-link-lib=msvcrt");
            
        }
    }
}

fn download_or_find_webui_root(out_dir: &str) -> String {
    match get_env_var("WEBUI_ROOT") {
        Ok(val) => {
            if true {
                panic!("WEBUI_ROOT is set to {}", val);
            } else {
                val
            }
        },
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
                println!("cargo:warning=Downloading webui library from {}", url);

                download_file(&url, &output_path).expect(&format!("Failed to download webui {} library", version));

                println!("cargo:warning=Downloaded webui library from {}", url);
            } else {
                println!("cargo:warning=Already downloaded webui library at {}", output_path);
            }

            let fname = Path::new(&output_path);
            let file = File::open(&fname).expect("Failed to open downloaded file");
            
            // Extract the zip file
            let mut zf = zip::ZipArchive::new(file).expect("Failed to open zip file");
            zf.extract(out_dir).unwrap();

            format!("{}/webui-{}", out_dir, triplet)
        }
    }
}