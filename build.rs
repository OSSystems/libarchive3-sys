extern crate pkg_config;

use std::env;

fn main() {
    let lib_dir = env::var("LIBARCHIVE_LIB_DIR").ok();
    let include_dir = env::var("LIBARCHIVE_INCLUDE_DIR").ok();

    if lib_dir.is_some() && include_dir.is_some() {
        println!("cargo:rustc-link-search=native={}", lib_dir.unwrap());
        println!("cargo:include={}", include_dir.unwrap());
        let mode = match env::var_os("LIBARCHIVE_STATIC") {
            Some(_) => "static",
            None => "dylib",
        };
        println!("cargo:rustc-flags=-l {0}=archive", mode);
    } else {
        match pkg_config::find_library("libarchive") {
            Ok(_) => (),
            Err(msg) => panic!("Unable to locate libarchive, err={:?}", msg),
        }
    }
}
