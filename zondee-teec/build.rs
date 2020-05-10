use bindgen::Builder;
use std::{env, path::PathBuf};

fn main() {
    let tr = env::var("TEEC_ROOT").expect("Set TEEC_ROOT environment variable");
    println!("cargo:rerun-if-changed=client.h");
    println!("cargo:rustc-link-lib=teec");
    println!("cargo:rustc-link-search={}/lib", tr);
    let bindings = Builder::default()
        .clang_arg(format!("-I{}/include", tr))
        .ctypes_prefix("libc")
        .derive_default(true)
        .header("teec.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .use_core()
        .generate()
        .expect("Unable to generate bindings");
    bindings
        .write_to_file(PathBuf::from(env::var("OUT_DIR").unwrap()).join("teec.rs"))
        .expect("Couldn't write bindings!");
}
