use bindgen::{Bindings, Builder};
use std::{env, path::PathBuf};

fn main() {
    let client_root = env::var("CLIENT_ROOT").expect("Set CLIENT_ROOT environment variable");
    let os_root = env::var("OS_ROOT").expect("Set OS_ROOT environment variable");

    println!("cargo:rerun-if-changed=client.h");
    println!("cargo:rustc-link-lib=teec");
    println!("cargo:rustc-link-search={}/lib", client_root);
    
    let ci = format!("-I{}/include", client_root);
    write_to_file(
        bindings(Builder::default().clang_arg(ci).header("client.h")),
        "client.rs",
    );
    
    println!("cargo:rerun-if-changed=os.h");
    println!("cargo:rustc-link-lib=static=mbedtls");
    println!("cargo:rustc-link-lib=static=utee");
    println!("cargo:rustc-link-lib=static=utils");
    println!("cargo:rustc-link-search={}/lib", os_root);

    let oi = format!("-I{}/include", os_root);
    write_to_file(
        bindings(Builder::default().clang_arg(oi).header("os.h")),
        "os.rs",
    );
}

fn bindings(builder: Builder) -> Bindings {
    builder
        .ctypes_prefix("libc")
        .derive_default(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .use_core()
        .generate()
        .expect("Unable to generate bindings")
}

fn write_to_file(bindings: Bindings, file: &str) {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join(file))
        .expect("Couldn't write bindings!");
}
