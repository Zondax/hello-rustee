use bindgen::Builder;
use std::{env, path::PathBuf};

fn main() {
    let ur = env::var("UTEE_ROOT").expect("Set UTEE_ROOT environment variable");
    println!("cargo:rerun-if-changed=os.h");
    println!("cargo:rustc-link-lib=static=mbedtls");
    println!("cargo:rustc-link-lib=static=utee");
    println!("cargo:rustc-link-lib=static=utils");
    println!("cargo:rustc-link-search={}/lib", ur);
    let bindings = Builder::default()
        .clang_arg(format!("-I{}/include", ur))
        .ctypes_prefix("libc")
        .derive_default(true)
        .header("utee.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .use_core()
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("utee.rs"))
        .expect("Couldn't write bindings!");
}
