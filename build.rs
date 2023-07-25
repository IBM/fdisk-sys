use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=fdisk");
    println!("cargo:rerun-if-changed=wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_type("fdisk_.*")
        .allowlist_function("fdisk_.*")
        .allowlist_var("FDISK_.*")
        .allowlist_var("LIBFDISK_.*")
        .allowlist_var("DOS_.*")
        .allowlist_var("GPT_.*")
        .allowlist_var("SGI_.*")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
