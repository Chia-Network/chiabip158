use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.cpp");

    cc::Build::new()
        .cpp(true)
        .std("c++14")
        .files([
            "../src/blockfilter.cpp",
            "../src/crypto/sha256.cpp",
            "../src/crypto/siphash.cpp",
            "../src/primitives/block.cpp",
            "../src/primitives/transaction.cpp",
            "../src/script/script.cpp",
            "../src/util/strencodings.cpp",
            "../src/util/bytevectorhash.cpp",
            "../src/uint256.cpp",
            "./wrapper.cpp",
        ])
        .warnings(false)
        .include("../src")
        .compile("chiabip158");

    let bindings = bindgen::Builder::default()
        .header("./wrapper.h")
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-I../src")
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
