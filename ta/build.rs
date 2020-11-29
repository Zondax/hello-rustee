use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use uuid::Uuid;

fn main() -> std::io::Result<()> {
    let out = &PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let ta_search_path = &PathBuf::from(env::var_os("UTEE_ROOT").expect("no CROSS_COMPILE env"));

    let _search_path = match env::var("ARCH") {
        Ok(ref v) if v == "arm" => {
            File::create(out.join("ta.lds"))?.write_all(include_bytes!("ta_arm.lds"))?;
            Path::new(&ta_search_path).join("/lib")
        }
        _ => {
            File::create(out.join("ta.lds"))?.write_all(include_bytes!("ta_aarch64.lds"))?;
            Path::new(&ta_search_path).join("/lib")
        }
    };
    //let search_path = Path::new(&search_path).join("/lib");
    //let search_path = Path::new(&search_path).join("/lib");
    println!("cargo:rustc-link-search={}", out.display());
    println!("cargo:rerun-if-changed=ta.lds");

    //println!("cargo:rustc-link-search={}/lib", search_path.display());
    Ok(())
}
