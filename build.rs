use std::env;
use std::path::PathBuf;

const ENV_STATIC: &str = "ULTRAHDR_STATIC";
const LIB_PATH: &str = "ULTRAHDR_LIB_PATH";

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let default_static = false;
    let static_build = match env::var(ENV_STATIC) {
        Ok(val) => val.parse().unwrap_or(default_static),
        Err(_) => default_static,
    };

    find_installed_lib(static_build);

    // TODO: build from source?

    if static_build {
        println!("cargo:rustc-link-lib=static=uhdr");
    } else {
        println!("cargo:rustc-link-lib=uhdr");
    }
}

fn find_installed_lib(static_build: bool) {
    if let Ok(path) = env::var(LIB_PATH) {
        println!("cargo::rustc-link-search=native={}", path);
        return;
    }
    #[cfg(not(target_os = "windows"))]
    let find_result = pkg_config::Config::new()
        .statik(static_build)
        .probe("libuhdr");
    #[cfg(target_os = "windows")]
    let find_result = vcpkg::find_package("uhdr");
    let lib = find_result.unwrap();

    // link path
    for path in lib.link_paths {
        println!("cargo::rustc-link-search=native={}", path.display());
    }
    // bindgen
    let mut header = None;
    for path in lib.include_paths {
        let ideal_header = path.join("ultrahdr_api.h");
        if ideal_header.exists() {
            header = Some(ideal_header);
            break;
        }
    }
    let Some(header) = header else {
        println!("cargo:warning=uhdr_api.h not found");
        std::process::exit(1);
    };
    let outdir = std::env::var("OUT_DIR").unwrap();
    let output = PathBuf::from(outdir).join("bindings.rs");
    bindgen::Builder::default()
        .header(header.to_str().unwrap().to_string())
        .generate()
        .unwrap()
        .write_to_file(output)
        .unwrap();
}
