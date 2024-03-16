use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.cpp");

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut src_dir = manifest_dir.join("cpp");
    if !src_dir.exists() {
        src_dir = manifest_dir
            .parent()
            .expect("can't access ../src")
            .join("src")
    }

    cc::Build::new()
        .cpp(true)
        .std("c++14")
        .files([
            src_dir.join("blockfilter.cpp"),
            src_dir.join("crypto/sha256.cpp"),
            src_dir.join("crypto/siphash.cpp"),
            src_dir.join("primitives/block.cpp"),
            src_dir.join("primitives/transaction.cpp"),
            src_dir.join("script/script.cpp"),
            src_dir.join("util/strencodings.cpp"),
            src_dir.join("util/bytevectorhash.cpp"),
            src_dir.join("uint256.cpp"),
            manifest_dir.join("wrapper.cpp"),
        ])
        .warnings(false)
        .include(src_dir.as_path())
        .compile("chiabip158");

    let bindings = bindgen::Builder::default()
        .header(manifest_dir.join("wrapper.h").to_str().unwrap())
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg(format!("-I{}", src_dir.to_str().unwrap()))
        .clang_arg("-std=c++14")
        .blocklist_item("GCSFilter.+")
        .blocklist_item("ByteVector.*")
        .opaque_type("GCSFilter")
        .opaque_type("std.*")
        .allowlist_type("GCSFilter")
        .allowlist_type("Slice")
        .allowlist_function("create_filter")
        .allowlist_function("encode_filter")
        .allowlist_function("filter_match")
        .allowlist_function("filter_match_any")
        .allowlist_function("free_slice")
        .allowlist_function("free_filter")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
