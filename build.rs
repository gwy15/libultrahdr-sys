use std::env;
use std::path::{Path, PathBuf};

const LIB_PATH_ENV: &str = "UHDR_LIB_PATH";
const HEADER_ENV: &str = "UHDR_HEADER";
const STATIC_ENV: &str = "UHDR_STATIC";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    find_installed_lib();

    // TODO: build from source?
}

fn find_installed_lib() {
    if let Ok(path) = env::var(LIB_PATH_ENV) {
        println!("cargo::rustc-link-search=native={}", path);
        if env::var(STATIC_ENV).is_ok() {
            println!("cargo::rustc-link-lib=static=uhdr");
        } else {
            println!("cargo::rustc-link-lib=uhdr");
        }
        let header = env::var(HEADER_ENV)
            .expect(format!("{} set, but {} not set", LIB_PATH_ENV, HEADER_ENV).as_str());
        bindgen(&PathBuf::from(header));
        return;
    }

    #[cfg(not(target_os = "windows"))]
    pkg_config::Config::new().probe("libjpeg").unwrap();
    #[cfg(target_os = "windows")]
    vcpkg::find_package("libjpeg-turbo").unwrap();

    #[cfg(not(target_os = "windows"))]
    let find_result = pkg_config::Config::new().probe("libuhdr");
    #[cfg(target_os = "windows")]
    let find_result = vcpkg::find_package("uhdr");
    let lib = find_result.unwrap();

    // bindgen
    let mut header = std::env::var(HEADER_ENV).ok().map(Into::into);
    if header.is_none() {
        for path in lib.include_paths {
            let ideal_header = path.join("ultrahdr_api.h");
            if ideal_header.exists() {
                header = Some(ideal_header);
                break;
            }
        }
    }
    let Some(header) = header else {
        println!("cargo:warning=uhdr_api.h not found");
        std::process::exit(1);
    };
    bindgen(&header);
}

fn bindgen(header: &Path) {
    let outdir = std::env::var("OUT_DIR").unwrap();
    let output = PathBuf::from(outdir).join("bindings.rs");
    bindgen::Builder::default()
        .header(header.to_str().unwrap().to_string())
        .default_enum_style(bindgen::EnumVariation::Rust {
            non_exhaustive: true,
        })
        .generate()
        .unwrap()
        .write_to_file(output)
        .unwrap();
}
