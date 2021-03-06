extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=ddsc");
    println!("cargo:rustc-link-lib=cdds-util");
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_arg("-I/usr/local/include")
        .clang_arg("-I/home/ros/workspace/zenoh_dds_dep_ws/install/include/")
        .clang_arg("-I/home/evshary/workspace/zenoh_dds_dep_ws/install/include/")
        .generate_comments(false)
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
