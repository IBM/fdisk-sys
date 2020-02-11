use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=fdisk");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .whitelist_type("fdisk_.*")
        .whitelist_function("fdisk_.*")
        .whitelist_var("FDISK_.*")
        .whitelist_var("LIBFDISK_.*")
        .whitelist_var("DOS_.*")
        .whitelist_var("GPT_.*")
        .whitelist_var("SGI_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
